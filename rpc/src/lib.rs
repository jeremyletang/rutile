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
    id: i64
}

pub trait JsonConvertible: serde::Deserialize + serde::Serialize {}

pub trait Service {
    fn __rpc_service_name(&self) -> &'static str;
    fn __serve_rpc_request(&mut self, c: Context, m: Message) -> bool;
}
