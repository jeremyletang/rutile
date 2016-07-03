// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use syntax::ast::*;
use syntax::codemap::Span;
use syntax::ext::base::ExtCtxt;
use syntax::ptr::P;

use aster::ident::ToIdent;
use aster::ty::TyPathBuilder;
use aster::path::PathBuilder;
use aster::ty::TyBuilder;

fn build_ty_from_string(span: &Span, s: String) -> P<Ty> {
    let path = Path::from_ident(span.clone(), s.to_ident());
    TyBuilder::new().build_path(path)
}

pub fn extract_codec_from_meta_item(cx: &mut ExtCtxt, mi: &MetaItem) -> Vec<P<Ty>> {
    match &mi.node {
        &MetaItemKind::List(_, ref l) => {
            l.iter().map(|e| {
                match &e.node {
                    &MetaItemKind::Word(ref s) => TyBuilder::new().id(s.to_string()),
                    &MetaItemKind::NameValue(ref v, ref lit) => {
                        match &lit.node {
                            &LitKind::Str(ref s, _) => build_ty_from_string(&lit.span, s.to_string()),
                            _ => cx.span_fatal(lit.span, "error rpc_service attribute element must be a string literal")
                        }
                    },
                    _ => cx.span_fatal(mi.span, "error rpc_service attribute element cannont be a list")
                }
            }).collect()
        },
        _ => cx.span_fatal(mi.span, "error rpc_service attribute must be a list")
    }
}
