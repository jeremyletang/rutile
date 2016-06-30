// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;

use std::collections::HashMap;

pub type Context = HashMap<String, String>;
pub type RutileError = Option<Box<::std::error::Error>>;

pub fn make_empty_context() -> Context {
    HashMap::new()
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Message {
    pub method: String,
    pub body: String,
    pub id: i64,
}

pub fn __decode_and_call<T1, T2, F>(c: &Context, m: &Message, mut f: F)
    where F: FnMut(T1, T2) -> RutileError,
    T1: JsonConvertible,
    T2: JsonConvertible {
    
    let t1 = T1::default();
    let t2 = T2::default();
    println!("thug life");
    f(t1, t2);
}

pub trait JsonConvertible: serde::Deserialize + serde::Serialize + Default {
    fn from_message(&mut self, m: &Message) {

    }

    fn to_message(&self, m: &mut Message) {

    }
}

pub trait Service: Send + Sync + 'static {
    fn __rpc_service_name(&self) -> &'static str;
    fn __rpc_list_methods(&self) -> Vec<String>;
    fn __rpc_serve_request(&self, c: Context, m: Message) -> bool;
}
