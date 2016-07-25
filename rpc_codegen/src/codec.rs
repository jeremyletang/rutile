// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use aster::ident::ToIdent;
use aster::ty::TyPathBuilder;
use aster::path::PathBuilder;
use aster::ty::TyBuilder;
use aster::path::IntoPath;
use syntax::ast::*;
use syntax::codemap::Span;
use syntax::ext::base::ExtCtxt;
use syntax::ptr::P;

pub fn split_string_to_path_segments(s: String) -> Vec<PathSegment> {
    s.split("::").map(|sub|
        PathSegment{
            identifier: sub.to_ident(),
            parameters: PathParameters::none(),
        }
    ).collect()
}

fn build_path_from_string(span: &Span, s: String) -> Path {
    // check two first
    let mut s_to_split = s.clone();
    let mut chars = s.chars();
    let mut global = false;
    if chars.next() == Some(':') && chars.next() == Some(':') {
        // split only the rest of the characters + set path to global
        global = true;
        s_to_split = chars.collect::<String>().to_string();
    }
    Path {
        span: span.clone(),
        global: global,
        segments: split_string_to_path_segments(s_to_split),
    }
}

pub fn extract_codec_from_meta_item(cx: &mut ExtCtxt, mi: &MetaItem) -> Vec<Path> {
    match &mi.node {
        &MetaItemKind::List(_, ref l) => {
            l.iter().map(|e| {
                match &e.node {
                    &MetaItemKind::Word(ref s) => s.to_string().into_path(),
                    &MetaItemKind::NameValue(ref v, ref lit) => {
                        match &lit.node {
                            &LitKind::Str(ref s, _) => {
                                if s.len() == 0 {
                                    cx.span_fatal(lit.span, "error rpc_service attribute string literal cannot be empty")
                                }
                                build_path_from_string(&lit.span, s.to_string())
                            },
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
