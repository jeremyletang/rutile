// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use hyper::header::ContentType;

use context::Context;

pub mod json_codec;

pub trait Message: Clone + Default + Sized {
    type I: Clone;
    fn get_method(&self) -> &str;
    fn get_body(&self) -> &Self::I;
    fn get_id(&self) -> i64;
    fn set_method(&mut self, method: &str);
    fn set_body(&mut self, body: &Self::I);
    fn set_id(&mut self, id: i64);
}

pub trait Codec<T>: Clone + Default + CodecBase {
    type M: Message + Clone;
    fn extract_method_from_raw(&self, s: &String) -> Option<String> {
        return self.method(s);
    }
    fn from_string(&self, &str) -> Option<T>;
    fn to_string(&self, &T) -> Option<String>;
    fn decode_message(&self, &String) -> Result<Box<Self::M>, String>;
}

pub trait CodecBase {
    fn empty() -> Self;
    fn method(&self, s: &str) -> Option<String>;
    fn content_type(&self) -> ContentType;
}

pub fn __decode_and_call<Request, Response, Error, F, C>(ctx: &Context, codec: &C, body: &String, mut f: F)
    where F: FnMut(&Context, <<C as Codec<Request>>::M as Message>::I) -> Result<Response, Error>,
    C: Codec<Request> + Codec<Response> + Codec<Error> {
    // FIXME(JEREMY): make error handling here
    let message = <C as Codec<Request>>::decode_message(codec, body).ok().unwrap();
    info!("dispatching message to method {}", message.get_method());
    let req = message.get_body().clone();
    let s = match f(ctx, req.clone()) {
        Ok(res) => codec.to_string(&res),
        Err(err) => codec.to_string(&err),
    }.expect("unable to convert response");
}
