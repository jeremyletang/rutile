// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use typemap::{CloneMap};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Context {
    pub metas: HashMap<String, String>,
    pub ext: CloneMap,
}

impl Context {
    pub fn new() -> Context {
        Context {
            metas: HashMap::new(),
            ext: CloneMap::custom(),
        }
    }

    pub fn with_metas(meta: HashMap<String, String>) -> Context {
        Context {
            metas: meta,
            ext: CloneMap::custom(),
        }
    }

    pub fn add_meta<S: Into<String>>(&mut self, k: S, v: S) {
        self.metas.insert(k.into(), v.into());
    }

    pub fn get_meta<S: Into<String>>(&self, k: S) -> Option<&String> {
        self.metas.get(&k.into())
    }
}
