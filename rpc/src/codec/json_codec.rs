// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel};
use serde::{Serialize, Deserialize};
use serde_json::{self, Value};
use std::error::Error;

use codec::{Codec, Message, CodecMethodExtract, CodecContentTypeExtract};

#[derive(Clone, Default, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct Dummy;

#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct JsonCodec {}

impl JsonCodec {
    pub fn new() -> JsonCodec {
        JsonCodec{}
    }
}

#[derive(Clone, Default, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct JsonMessage<T> where T: Default + Clone + Serialize + Deserialize {
    method: String,
    body: T,
    id: i64,
}

impl<T> Message for JsonMessage<T> where T: Default + Clone + Serialize + Deserialize{
    type I = T;
    fn get_method(&self) -> &str { &self.method }
    fn get_body(&self) -> &Self::I { &self.body }
    fn get_id(&self) -> i64 { self.id }
    fn set_method(&mut self, method: &str) { self.method = method.to_string(); }
    fn set_body(&mut self, body: &Self::I) { self.body = body.clone(); }
    fn set_id(&mut self, id: i64) { self.id = id; }
}

impl CodecMethodExtract for JsonCodec {
    fn extract(&self, s: &String) -> Option<String> {
        let value: Value = serde_json::from_str(&*s).unwrap();
        Some(value.find("method").unwrap().as_string().unwrap().to_string())
    }
}

impl CodecContentTypeExtract for JsonCodec {
    fn content_type(&self) -> ContentType {
        ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![]))
    }
}


impl<T> Codec<T> for JsonCodec
    where T: Serialize + Deserialize + Clone + Default {
    type M = JsonMessage<T>;

    fn from_string(&self, s: &str) -> Option<T> {
        serde_json::from_str(&s).ok()
    }

    fn to_string(&self, t: &T) -> Option<String> {
        serde_json::to_string(&t).ok()
    }

    fn decode_message(&self, raw_message: &String) -> Result<Box<Self::M>, String> {
        match serde_json::from_str(&raw_message) {
            Ok(t) => Ok(t),
            Err(e) => Err(e.description().to_string())
        }
    }
}

// pub fn __decode_and_call<Request, Response, Error, F, C>(ctx: &Context, codec: &C, body: &String, mut f: F)
//     where F: FnMut(&Context, Request) -> Result<Response, Error>,
//     Request: Default + Deserialize + Serialize,
//     C: Codec<Request> + Codec<Response> + Codec<&Message<Request>> {
//     let req = Request::default();
//     println!("thug life");
//     f(ctx, req);
// }
