// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use hyper::header::ContentType;

use context::Context;
use service::ServeRequestError;

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
    fn extract_method_from_raw(&self, s: &String) -> Result<String, String> {
        return self.method(s);
    }
    fn from_string(&self, &str) -> Result<T, String>;
    fn to_string(&self, &T) -> Result<String, String>;
    fn decode_message(&self, &String) -> Result<Box<Self::M>, String>;
}

pub trait CodecBase {
    fn empty() -> Self;
    fn method(&self, s: &str) -> Result<String, String>;
    fn content_type(&self) -> ContentType;
}

pub fn __decode_and_call<Request, Response, Error, F, C>(ctx: &Context, codec: &C, body: &String, mut f: F)
    -> Result<(), ServeRequestError>
    where F: FnMut(&Context, <<C as Codec<Request>>::M as Message>::I) -> Result<Response, Error>,
    C: Codec<Request> + Codec<Response> + Codec<Error> {

    let message = match <C as Codec<Request>>::decode_message(codec, body) {
        Ok(m) => m,
        Err(e) => return Err(ServeRequestError::InvalidBody(e))
    };
    info!("dispatching message to method {}", message.get_method());
    let _ = match f(ctx,  message.get_body().clone()) {
        Ok(res) => codec.to_string(&res),
        Err(err) => codec.to_string(&err),
    }.expect("unable to convert response");
    return Ok(())
}
