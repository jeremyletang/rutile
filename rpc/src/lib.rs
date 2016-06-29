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


#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    method: String,
    body: String,
    id: i64,
}

pub trait JsonConvertible: serde::Deserialize + serde::Serialize {}

pub trait Service {
    fn __rpc_service_name(&self) -> &'static str;
    fn __rpc_list_methods(&self) -> Vec<String>;
    fn __rpc_serve_request(&mut self, c: Context, m: Message) -> bool;
}
