// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(rustc_private, plugin)]
#![plugin(quasi_macros)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![allow(unused_imports, unused_variables)]

extern crate rpc;
extern crate syntax;
extern crate rustc_plugin;
extern crate quasi;
extern crate aster;

use aster::ident::ToIdent;
use aster::item::ItemBuilder;
use aster::expr::ExprBuilder;
use aster::lit::LitBuilder;
use aster::path::IntoPath;
use aster::str::ToInternedString;
use aster::ty::TyBuilder;
use aster::ty::TyPathBuilder;
use quasi::ToTokens;
use syntax::ast::*;
use syntax::codemap::{Span, spanned, BytePos};
use syntax::ext::base::{Annotatable, ExtCtxt, MultiModifier};
use syntax::ext::build::AstBuilder;
use syntax::parse::token::InternedString;
use syntax::ptr::P;

mod codec;

fn camel_to_snake(mut camel: String) -> String {
    let mut snake = String::new();
    if camel.len() > 0 {
        snake.push(camel.remove(0).to_lowercase().next().unwrap())
    }
    for c in camel.chars() {
        if c.is_uppercase() {
            snake.push('_');
        }
        snake.push(c.to_lowercase().next().unwrap())
    }
    return snake;
}

fn methods_raw_to_ident(service_name: &str,
                        methods_raw: &Vec<(Ident, Vec<P<Ty>>, Vec<P<Ty>>)>)
                        -> Vec<Ident> {
    methods_raw.iter()
        .map(|&(i, _, _)| {
            let s = service_name.to_string() + "_" + &syntax::print::pprust::ident_to_string(i);
            let s = s.replace(".", "_").to_uppercase();
            s.to_ident()
        }).collect()
}

fn make_service_name(cx: &mut ExtCtxt, ty_kind: &syntax::ast::TyKind) -> String {
    let crate_name = cx.ecfg.crate_name.to_string() + ".";
    let mod_path = cx.mod_path_stack
        .iter()
        .fold("".to_string(), |acc, seg| acc + seg + ".");
    let mut ty_name = match ty_kind {
        &TyKind::Path(_, ref p) => {
            p.segments.iter().fold("".to_string(), |acc, seg| {
                acc + &camel_to_snake(syntax::print::pprust::ident_to_string(seg.identifier)) + "."
            })
        }
        _ => unreachable!(),
    };
    ty_name.pop();
    crate_name + &mod_path + &ty_name
}

fn make_client_struct_name(cx: &mut ExtCtxt, ty_kind: &syntax::ast::TyKind) -> String {
    let ty_name = match ty_kind {
        &TyKind::Path(_, ref p) => {
            syntax::print::pprust::ident_to_string(p.segments.iter().last().unwrap().identifier)
        }
        _ => unreachable!(),
    };
    ty_name + "Client"
}

fn extract_method_args(sig: &syntax::ast::MethodSig) -> Vec<P<Ty>> {
    sig.decl.inputs.iter().map(|a| a.ty.clone()).collect()
}

fn extract_method_return_type_parameters(sig: &syntax::ast::MethodSig) -> Vec<P<Ty>> {
    match &sig.decl.output {
        &FunctionRetTy::Ty(ref ty) => {
            match (*ty).node {
                TyKind::Path(_, ref p) => {
                        let seg_len = p.segments.len();
                        match p.segments[seg_len-1].parameters {
                            PathParameters::AngleBracketed(ref pp) => {
                                pp.types.to_vec()
                            },
                            _ => vec![]
                        }
                },
                _ => {
                    vec![]
                }
            }
        }
        _ => {
            vec![]
        },
    }
}

fn make_service_methods_list(cx: &mut ExtCtxt, items: &Vec<ImplItem>)
    -> Vec<(Ident, Vec<P<Ty>>, Vec<P<Ty>>)> {
    let mut methods = vec![];
    for i in items {
        match i.node {
            ImplItemKind::Method(ref sig, _) => {
                let args = extract_method_args(sig);
                let ret_ty_params = extract_method_return_type_parameters(sig);
                if ret_ty_params.len() == 0 {
                    cx.span_err(i.span, "service methods must return Result<_, _>");
                }
                methods.push((i.ident, args, ret_ty_params));
            }
            _ => {
                // nothing to do with non methods kinds
            }
        };
    }

    return methods;
}

fn methods_raw_to_str_literals_list(service_name: &str,
                                    methods_raw: &Vec<(Ident, Vec<P<Ty>>, Vec<P<Ty>>)>)
                                    -> Vec<Lit> {
    methods_raw.iter()
        .map(|&(i, _, _)| {
            let en = service_name.to_string() + "." + &syntax::print::pprust::ident_to_string(i);
            (*LitBuilder::new().str(&*en)).clone()
        }).collect()
}

fn make_list_endpoints_fn_expr(cx: &mut ExtCtxt,
                               service_name: &str,
                               methods_raw: &Vec<(Ident, Vec<P<Ty>>, Vec<P<Ty>>)>)
                               -> P<Expr> {

    let endpoint_names = methods_raw_to_str_literals_list(service_name, methods_raw).into_iter();
    quote_expr!(cx, vec![$($endpoint_names.to_string(),)*])
}

fn make_supported_codecs_fn_expr(cx: &mut ExtCtxt,
                                 mut codec_paths: Vec<Path>)
                                 -> Stmt {
    for p in &mut codec_paths {
        p.segments.push(
            PathSegment{
                identifier: "default".to_ident(),
                parameters: PathParameters::none(),
        });
    }
    let paths_iter = codec_paths.into_iter();
    quote_stmt!(cx,
        vec![$($paths_iter().content_type(),)*]
    ).unwrap()
}

fn make_endpoints_match_fn_expr(cx: &mut ExtCtxt,
                                service_name: &str,
                                methods_raw: &Vec<(Ident, Vec<P<Ty>>, Vec<P<Ty>>)>)
                                -> Vec<P<Block>> {
    methods_raw.iter()
        .map(|&(i, ref args, ref retty)| {
            let ref req = args[2];
            let ref ret_ok = retty[0];
            let ref ret_err = retty[1];
            let en = service_name.to_string() + "." + &syntax::print::pprust::ident_to_string(i);
            quote_block!(cx, {
                let f = |ctx: &::rpc::context::Context, r: $req| -> Result<$ret_ok, $ret_err> {self.$i(ctx, r)};
                ::rpc::codec::__decode_and_call::<$req, $ret_ok, $ret_err, _, ::rpc::codec::json_codec::JsonCodec>(&ctx, &codec, &body, f, res)
            }).unwrap()
        }).collect()
}

fn make_client(cx: &mut ExtCtxt,
              ty: &P<Ty>,
              generics: &Generics,
              methods: &Vec<ImplItem>,
              codec_paths: &Vec<Path>)
              -> Vec<P<Item>> {

    let client_struct_name = make_client_struct_name(cx, &(*ty).node);
    let client_struct_name_expr = client_struct_name.to_ident();

    vec![
    quote_item!(cx,
        pub struct $client_struct_name_expr {
            timeout: ::std::time::Duration
        }
    ).unwrap(),
    quote_item!(cx,
        impl $client_struct_name_expr {
            pub fn new() -> $client_struct_name_expr {
                $client_struct_name_expr {
                    timeout: ::std::time::Duration::new(5, 0),
                }
            }
            pub fn with_timeout(d: ::std::time::Duration) -> $client_struct_name_expr {
                $client_struct_name_expr {
                    timeout: d,
                }
            }
        }
    ).unwrap()
    ]
}

fn make_service_trait_impl_item(cx: &mut ExtCtxt,
                                ty: &P<Ty>,
                                generics: &Generics,
                                methods: &Vec<ImplItem>,
                                codec_paths: &Vec<Path>)
                                -> Option<P<Item>> {
    let service_name = make_service_name(cx, &(*ty).node);
    let service_name_expr = ExprBuilder::new().str(&*service_name);

    let methods_raw = make_service_methods_list(cx, &methods);
    let list_endpoints_fn_expr = make_list_endpoints_fn_expr(cx, &service_name, &methods_raw);

    let method_name_lits = methods_raw_to_str_literals_list(&service_name, &methods_raw).into_iter();
    let match_fn_exprs = make_endpoints_match_fn_expr(cx, &service_name, &methods_raw).into_iter();

    let list_supported_codecs_expr = make_supported_codecs_fn_expr(cx, codec_paths.clone());

    let where_clauses = generics.where_clause.clone();

    quote_item!(cx,
        impl$generics ::rpc::service::Service for $ty $where_clauses {
            default fn __rpc_service_name(&self) ->  &'static str{
                return $service_name_expr;
            }
            default fn __rpc_list_methods(&self) -> Vec<String> {
                $list_endpoints_fn_expr
            }
            default fn __rpc_list_supported_codecs(&self) -> Vec<::rpc::ext_exports::ContentType> {
                use ::rpc::codec::CodecBase;
                $list_supported_codecs_expr
            }
            default fn __rpc_serve_request(&self, ctx: ::rpc::context::Context,
                                                  req: &mut ::rpc::transport::TransportRequest,
                                                  res: &mut ::rpc::transport::TransportResponse)
                                                  -> Result<(), ::rpc::service::ServeRequestError> {
                use ::rpc::codec::{Codec, CodecBase};
                let mut body = String::new();
                let _ = req.read_to_string(&mut body);
                let codec = ::rpc::codec::json_codec::JsonCodec::default();
                let method = match codec.method(&body) {
                    Ok(s) => s,
                    Err(e) => return Err(::rpc::service::ServeRequestError::NoMethodProvided(e))
                };
                match &*method {
                    $($method_name_lits => $match_fn_exprs,)*
                    _ => return Err(::rpc::service::ServeRequestError::UnrecognizedMethod(method))
                }
            }
        }
    )
}

fn make_endpoints_impl_item(cx: &mut ExtCtxt,
                           ty: &P<Ty>,
                           generics: &Generics,
                           methods: &Vec<ImplItem>) -> Vec<P<Item>> {
    let where_clauses = generics.where_clause.clone();
    let service_name = make_service_name(cx, &(*ty).node);
    let service_name_expr = ExprBuilder::new().str(&*service_name);
    let methods_raw = make_service_methods_list(cx, &methods);
    let method_name_lits = methods_raw_to_str_literals_list(&service_name, &methods_raw).into_iter();
    let method_raw_idents = methods_raw_to_ident(&service_name, &methods_raw.clone()).into_iter();
    let items: Vec<P<Item>> = method_raw_idents.into_iter().zip(method_name_lits.into_iter()).map(|(a, b)|
    quote_item!(cx, pub const $a: &'static str = $b;).unwrap()).collect();
    return items;
    // items.push(quote_item!(cx, pub const SERVICE_NAME: &'static str = $service_name_expr;).unwrap());
}

fn has_rpc_methods_attribute(attrs: &Vec<Attribute>) -> bool {
    for a in attrs {
        match &a.node.value.node {
            &MetaItemKind::Word(ref is) => {
                // only check this as for now rpc_methods attribute can only be a word
                if *is == "rpc_methods" {
                    return true;
                }
            },
            _ => {}
        }
    }

    return false;
}

fn find_mod_impl(m: &Mod) -> (Vec<ItemKind>, Vec<P<Item>>) {
    let mut items = vec![];
    let mut impls = vec![];
    for i in &m.items {
        match i.node {
            ItemKind::Impl(_, _, _, _, _, _) => {
                if has_rpc_methods_attribute(&i.attrs) {
                    impls.push(i.node.clone());
                }
                items.push(i.clone());
            },
            _ => items.push(i.clone())
        }
    }
    (impls, items)
}

fn generate_rpc_service(cx: &mut ExtCtxt, impls: &Vec<ItemKind>, codec_paths: &Vec<Path>) -> Vec<P<Item>> {
    let mut items = vec![];
    for imp in impls {
        items.append(
            &mut match imp {
                &ItemKind::Impl(_, _, ref generics, _, ref ty, ref methods) => {
                    let impl_item = make_service_trait_impl_item(cx, ty, generics, methods, codec_paths);
                    let mut client_item = make_client(cx, ty, generics, methods, codec_paths);
                    let mut impl_endpoints = make_endpoints_impl_item(cx, ty, generics, methods);
                    impl_endpoints.push(impl_item.unwrap());
                    impl_endpoints.append(&mut client_item);
                    impl_endpoints
                },
                _ => vec![]
            }
        )
    }
    return items;
}

fn expand_rpc_service(cx: &mut ExtCtxt,
                      span: Span,
                      meta_item: &MetaItem,
                      annotatable: Annotatable)
                      -> Vec<Annotatable> {
    let codec_paths = codec::extract_codec_from_meta_item(cx, meta_item);
    // let mut items = make_use_items(cx, codec_paths.clone());
    let mut items: Vec<P<Item>> = vec![];
    match annotatable {
        Annotatable::Item(ref i) => {
            match &(*i).node {
                &ItemKind::Mod(ref m) => {
                    let (impls, mut base_items) = find_mod_impl(&m);
                    if impls.len() == 0 {
                        cx.span_fatal(span, "cannot found struct or enum impls");
                    }
                    items.append(&mut base_items);
                    let mut _mod = m.clone();
                    items.append(&mut generate_rpc_service(cx, &impls, &codec_paths));
                    _mod.items = items;
                    let item = P(Item {
                        ident: i.ident.clone(),
                        attrs: i.attrs.clone(),
                        id: DUMMY_NODE_ID,
                        node: ItemKind::Mod(_mod),
                        vis: Visibility::Public,
                        span: span,
                    });
                    println!("{}", syntax::print::pprust::item_to_string(&item.clone().unwrap()));
                    return vec![Annotatable::Item(item)];
                }
                _ => {
                    cx.span_fatal(span,
                                  "#[rpc_service(...)] may only be applied modules");
                }
            }
        },
        _ => {
            cx.span_fatal(span,
                          "#[rpc_service(...)] may only be applied modules");
        }
    }
}

// this is only used to prevent warning while using the rpc_methods attribute
fn expand_rpc_methods(_: &mut ExtCtxt,
                      _: Span,
                      _: &MetaItem,
                      annotatable: Annotatable)
                      -> Vec<Annotatable> {
    vec![annotatable]
}

pub fn register(reg: &mut rustc_plugin::Registry) {
    reg.register_syntax_extension(syntax::parse::token::intern("rpc_service"),
                                  MultiModifier(Box::new(expand_rpc_service)));
    // to prevent warning for unused attributes
    reg.register_syntax_extension(syntax::parse::token::intern("rpc_methods"),
                                  MultiModifier(Box::new(expand_rpc_methods)));
}
