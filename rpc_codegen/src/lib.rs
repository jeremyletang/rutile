#![feature(rustc_private, plugin)]
#![plugin(quasi_macros)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![allow(unused_imports, unused_variables)]

extern crate syntax;
extern crate rustc_plugin;
extern crate quasi;

use syntax::ast::*;
// use syntax::ast::{self, Ident, MetaItem};
use syntax::codemap::Span;
use syntax::ext::base::{Annotatable, ExtCtxt};
use syntax::ptr::P;
use syntax::ext::build::AstBuilder;

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
                    println!("Items: {:?}", methods);
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
                            fn rpc_service_name(&self) ->  &'static str{
                                return "Test";
                            }
                            fn serve_rpc_request(&mut self, c: ::rpc::Context, m: ::rpc::Message) -> bool {
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
