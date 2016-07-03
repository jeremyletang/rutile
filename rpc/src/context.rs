// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use hyper::header::Headers;
use typemap::{CloneMap, TypeMap};

#[derive(Clone)]
pub struct Context {
    pub headers: Headers,
    pub ext: CloneMap,
}

impl Context {
    pub fn new() -> Context {
        Context {
            headers: Headers::new(),
            ext: CloneMap::custom(),
        }
    }
}
