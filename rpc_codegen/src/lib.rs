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

fn expand_rpc_service(cx: &mut ExtCtxt,
    span: Span,
    meta_item: &MetaItem,
    annotatable: &Annotatable,
    push: &mut FnMut(Annotatable)
) {
    match annotatable {
        &Annotatable::Item(ref i) => {
            match &(*i).node {
                &ItemKind::Impl(_, _, _, _, ref ty, ref items) => {
                    println!("this is an impl");
                    println!("TyKind: {:?}", (*ty).node);
                    println!("Items: {:?}", items);
                    let impl_item = quote_item!(cx,
                        impl Test {
                            pub fn list(&self) -> String{
                                return "mouhahaha".to_string();
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
