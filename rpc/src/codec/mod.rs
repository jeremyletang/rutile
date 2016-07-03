// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use context::Context;
use serde;

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

pub trait Codec<T>: Clone + Default + MethodExtract + ContentType {
    type M: Message + Clone;
    fn extract_method_from_raw(&self, s: &String) -> Option<String> {
        return self.extract(s);
    }
    fn from_string(&self, &str) -> Option<T>;
    fn to_string(&self, &T) -> Option<String>;
    fn decode_message(&self, &String) -> Option<Box<Self::M>>;
}

pub trait MethodExtract {
    fn extract(&self, s: &String) -> Option<String>;
}

pub trait ContentType {
    fn content_type(&self) -> &str;
}

pub fn __decode_and_call<Request, Response, Error, F, C>(ctx: &Context, codec: &C, body: &String, mut f: F)
    where F: FnMut(&Context, <<C as Codec<Request>>::M as Message>::I) -> Result<Response, Error>,
    C: Codec<Request> + Codec<Response> + Codec<Error> {
    let message = <C as Codec<Request>>::decode_message(codec, body).expect("unable to extract message");
    info!("dispatching message to method {}", message.get_method());
    let req = message.get_body().clone();
    let s = match f(ctx, req.clone()) {
        Ok(res) => codec.to_string(&res),
        Err(err) => codec.to_string(&err),
    }.expect("unable to convert response");
}
