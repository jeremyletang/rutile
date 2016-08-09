// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_json;
extern crate rpc;

use rpc::mime::{Mime, TopLevel, SubLevel};
use rpc::{Codec, CodecBase, DefaultMessage};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::error::Error;

#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct JsonCodec;

impl CodecBase for JsonCodec {
    fn method(&self, body: &[u8]) -> Result<String, String> {
        let value: Value = match serde_json::from_slice(body) {
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
    type M = DefaultMessage<T>;

    fn decode(&self, buf: &[u8]) -> Result<Box<Self::M>, String> {
        match serde_json::from_slice(buf) {
            Ok(t) => Ok(t),
            Err(e) => Err(e.description().to_string())
        }
    }

    fn encode(&self, body: &T, method: &str, id: u64) -> Result<Vec<u8>, String> {
        let m = DefaultMessage {
            method: method.to_string(),
            body: Some(body.clone()),
            id: id,
        };
        match serde_json::to_vec(&m) {
            Ok(s) => Ok(s),
            Err(e) => Err(e.description().to_string())
        }
    }
}
