// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use context::Context;
use serde;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Message {
    pub method: String,
    pub body: String,
    pub id: i64,
}

pub trait Codec: Clone {
    fn ty_from_message<T>(&self, &Message) -> Option<T>;
    fn ty_to_message<T>(&self, &T) -> Option<Message>;
}

pub trait JsonConvertible: serde::Deserialize + serde::Serialize + Default {
    fn from_message(&mut self, m: &Message) {

    }

    fn to_message(&self, m: &mut Message) {

    }
}


pub fn __decode_and_call<Request, Response, Error, F>(c: &Context, m: &Message, mut f: F)
    where F: FnMut(&Context, Request) -> Result<Response, Error>,
    Request: Default {
    let req = Request::default();
    println!("thug life");
    f(c, req);
}
