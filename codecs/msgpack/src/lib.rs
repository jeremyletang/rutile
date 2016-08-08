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
extern crate serde;
extern crate rmp;
extern crate rmp_serde;
extern crate rpc;

use std::io::Cursor;
use serde::{Serialize, Deserialize};
use std::error::Error;
use rmp_serde::{Serializer, Deserializer};

use rpc::{Codec, CodecBase, DefaultMessage};
use rpc::mime::{Mime, TopLevel, SubLevel};

#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct MsgpCodec {}

impl MsgpCodec {
    pub fn new() -> MsgpCodec {
        MsgpCodec{}
    }
}

impl CodecBase for MsgpCodec {
    fn method(&self, body: &[u8]) -> Result<String, String> {
        // msgpack encode complex value as array type.
        // so we try to decode the body as a msqpack array
        match rmp::decode::value::read_value(&mut &body[..]) {
            Ok(v) => {
                match v.as_array() {
                    Some(a) => {
                        // the array must explictly have a size of 3
                        if a.len() != 3 {
                            return Err("invalid msgpack message, expected array of size 3".to_string());
                        }
                        // first value of the array must be string
                        // this string will be the method
                        match a[0].as_str() {
                            Some(s) => {
                                Ok(s.to_string())
                            },
                            None => Err("invalid msgpack message, expected string at row 0".to_string()),
                        }
                    },
                    None => Err("invalid msgpack message, expected array".to_string()),
                }
            },
            Err(e) => Err("invalid msgpack message, unable to read value from the body".to_string()),
        }
    }

    fn content_type(&self) -> Mime {
        Mime(TopLevel::Application, SubLevel::Msgpack, vec![])
    }
}

impl<T> Codec<T> for MsgpCodec
    where T: Serialize + Deserialize + Clone {
    type M = DefaultMessage<T>;

    fn decode(&self, buf: &[u8]) -> Result<Box<Self::M>, String> {
        println!("decode raw message");
        let cur = Cursor::new(&buf[..]);
        let mut de = Deserializer::new(cur);
        let actual: Result<Self::M, _> = Deserialize::deserialize(&mut de);
        match actual {
            Ok(t) => Ok(Box::new(t)),
            Err(e) => Err(e.description().to_string())
        }
    }

    fn encode(&self, body: &T, method: &str, id: u64) -> Result<Vec<u8>, String> {
        let m = DefaultMessage {
            method: method.to_string(),
            body: Some(body.clone()),
            id: id,
        };
        let mut buf = vec![];
        match m.serialize(&mut Serializer::new(&mut buf)) {
            Ok(_) => Ok(buf),
            Err(e) => Err(e.description().to_string())
        }
    }
}
