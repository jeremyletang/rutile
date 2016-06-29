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

use syntax::ast::*;
use aster::path::IntoPath;
use syntax::ext::quote::rt::ToTokens;
use aster::ident::ToIdent;
use aster::ty::TyPathBuilder;
use aster::str::ToInternedString;
use aster::expr::ExprBuilder;
// use syntax::ast::{self, Ident, MetaItem};
use syntax::codemap::Span;
use syntax::ext::base::{Annotatable, ExtCtxt};
use syntax::ptr::P;
use syntax::ext::build::AstBuilder;

fn make_service_name(cx: &mut ExtCtxt, ty_kind: &syntax::ast::TyKind) -> String {
    let crate_name = cx.ecfg.crate_name.to_string() + ".";
    let mod_path = cx.mod_path_stack.
        iter().fold("".to_string(), |acc, seg| acc + seg + ".");
    let mut ty_name = match ty_kind {
        &TyKind::Path(_, ref p) => {
            p.segments.iter().fold("".to_string(), |acc, seg| {
                acc + &syntax::print::pprust::ident_to_string(seg.identifier) + "."
            })
        },
        _ => unreachable!(),
    };
    ty_name.pop();
    crate_name + &mod_path + &ty_name
}

fn make_service_methods_list(cx: &mut ExtCtxt, items: &Vec<ImplItem>) -> Vec<(Ident, Vec<P<Ty>>)> {
    // make rpc::RutileError
    // let error_ty = cx.ty_path("::rpc::RutileError".into_path());
    // println!("ty: {:?}", error_ty);
        // TyPathBuilder::new()
        // .path()
        // .segment("rpc").build()
        // .segment("RutileError")
        // .build());

    // .build();
    let mut methods = vec![];
    for i in items {
        let ident = i.ident;
        let mut arguments = Vec::<P<Ty>>::new();
        match i.node {
            ImplItemKind::Method(ref sig, _) => {
                // get arguments list
                for a in &sig.decl.inputs {
                    arguments.push(a.ty.clone());
                }
                match &sig.decl.output {
                    &FunctionRetTy::Ty(_) => {},
                    _ => cx.span_err(i.span, "service methods must return RutileError")
                }
                // println!("method: {}, sig: {:?}", name, sig)
            },
            _ => { /* nothing to do with non mathods kinds */ },
        };
        methods.push((ident, arguments));
    }

    return methods;
}

fn expand_rpc_service(cx: &mut ExtCtxt,
    span: Span,
    meta_item: &MetaItem,
    annotatable: &Annotatable,
    push: &mut FnMut(Annotatable)
) {
    match annotatable {
        &Annotatable::Item(ref i) => {
            match &(*i).node {
                &ItemKind::Impl(_, _, ref generics, _, ref ty, ref methods) => {

                    let service_name = ExprBuilder::new()
                        .str(&*make_service_name(cx, &(*ty).node));

                    make_service_methods_list(cx, &methods);
                    // let service_methods = make_service_methods_list(methods);


                    let mut exprs = Vec::new();
                    exprs.push(quote_stmt!(cx,
                        if true == true {
                            println!("thug life");
                        }));
                    exprs.push(quote_stmt!(cx,
                        if false == true {
                            println!("yolo");
                        }));
                    let exprs = exprs.into_iter();
                    let impl_item = quote_item!(cx,
                        impl$generics ::rpc::Service for $ty {
                            fn __rpc_service_name(&self) ->  &'static str{
                                return $service_name;
                            }
                            fn __serve_rpc_request(&mut self, c: ::rpc::Context, m: ::rpc::Message) -> bool {
                                $($exprs)*
                                return true;
                            }

                        }
                    ).unwrap();
                    push(Annotatable::Item(impl_item));
                },
                _ => cx.span_err((*i).span, "#[rpc_service(...)] may only be applied to struct or enum impls"),
            }
        },
        _ => cx.span_err(span, "#[rpc_service(...)] may only be applied to struct or enum impls")
    }
}

pub fn register(reg: &mut rustc_plugin::Registry) {
    reg.register_syntax_extension(
        syntax::parse::token::intern("rpc_service"),
        syntax::ext::base::MultiDecorator(
            Box::new(expand_rpc_service)));
}
