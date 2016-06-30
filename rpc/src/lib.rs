// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;

use context::Context;

pub mod context;
pub mod codec;
pub mod client;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Message {
    pub method: String,
    pub body: String,
    pub id: i64,
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
