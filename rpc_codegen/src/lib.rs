#![feature(rustc_private, plugin)]
#![plugin(quasi_macros)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate syntax;
extern crate rustc_plugin;

use syntax::ast::{self, Ident, MetaItem};
use syntax::codemap::Span;
use syntax::ext::base::{Annotatable, ExtCtxt};
use syntax::ptr::P;

fn expand_derive_service(cx: &mut ExtCtxt,
    span: Span,
    meta_item: &MetaItem,
    annotatable: &Annotatable,
    push: &mut FnMut(Annotatable)
) {}

pub fn register(reg: &mut rustc_plugin::Registry) {
    reg.register_syntax_extension(
        syntax::parse::token::intern("derive_Service"),
        syntax::ext::base::MultiDecorator(
            Box::new(expand_derive_service)));
}
