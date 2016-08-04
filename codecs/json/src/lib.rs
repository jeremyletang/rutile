// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate log;
extern crate mime;
extern crate serde;
extern crate serde_json;
extern crate rpc;

use mime::{Mime, TopLevel, SubLevel};
use rpc::{Codec, Message, CodecBase};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::error::Error;

#[derive(Clone, Default, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct Dummy;

#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct JsonCodec {
    body: String
}


#[derive(Clone, Default, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct JsonMessage<T> where T: Clone + Serialize + Deserialize {
    method: String,
    body: Option<T>,
    id: u64,
}

impl<T> Message for JsonMessage<T> where T: Clone + Serialize + Deserialize{
    type I = T;
    fn get_method(&self) -> &str { &self.method }
    fn get_body(&self) -> &Self::I { &self.body.as_ref().unwrap() }
    fn get_id(&self) -> u64 { self.id }
    fn set_method(&mut self, method: &str) { self.method = method.to_string(); }
    fn set_body(&mut self, body: &Self::I) { self.body = Some(body.clone()); }
    fn set_id(&mut self, id: u64) { self.id = id; }
}

impl JsonCodec {
    pub fn new() -> JsonCodec {
        JsonCodec {
            body: "".to_string()
        }
    }
}

impl CodecBase for JsonCodec {
    fn method(&self, body: &str) -> Result<String, String> {
        let value: Value = match serde_json::from_str(body) {
            Ok(v) => v,
            Err(e) => return Err(format!("invalid json, {}", e))
        };
        match value.find("method") {
            Some(v) => {
                match v.as_string() {
                    Some(s) => Ok(s.to_string()),
                    None => Err("method field found but not a string".to_string())
                }
            },
            None => Err("not method field found".to_string())
        }
    }

    fn content_type(&self) -> Mime {
        Mime(TopLevel::Application, SubLevel::Json, vec![])
    }
}

impl<T> Codec<T> for JsonCodec
    where T: Serialize + Deserialize + Clone {
    type M = JsonMessage<T>;

    fn from_string(&self, s: &str) -> Result<T, String> {
        match serde_json::from_str(&s) {
            Ok(t) => Ok(t),
            Err(e) => Err(e.description().to_string())
        }
    }

    fn to_string(&self, t: &T) -> Result<String, String> {
        match serde_json::to_string(&t) {
            Ok(s) => Ok(s),
            Err(e) => Err(e.description().to_string())
        }
    }

    fn decode_message(&self, raw_message: &str) -> Result<Box<Self::M>, String> {
        match serde_json::from_str(raw_message) {
            Ok(t) => Ok(t),
            Err(e) => Err(e.description().to_string())
        }
    }

    fn encode_message(&self, body: &T, method: &str, id: u64) -> Result<String, String> {
        let json_message = JsonMessage{
            method: method.to_string(),
            body: Some(body.clone()),
            id: id,
        };
        match serde_json::to_string(&json_message) {
            Ok(s) => Ok(s),
            Err(e) => Err(e.description().to_string())
        }
    }
}
