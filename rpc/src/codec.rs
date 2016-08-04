// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use mime::Mime;

use context::Context;
use handler::ServeRequestError;
use transport::TransportResponse;

pub trait Message: Clone + Sized {
    type I: Clone;
    fn get_method(&self) -> &str;
    fn get_body(&self) -> &Self::I;
    fn get_id(&self) -> u64;
    fn set_method(&mut self, method: &str);
    fn set_body(&mut self, body: &Self::I);
    fn set_id(&mut self, id: u64);
}

pub trait Codec<T>: Clone + CodecBase {
    type M: Message + Clone;
    fn extract_method_from_raw(&self, s: &String) -> Result<String, String> {
        return self.method(s);
    }
    fn from_string(&self, &str) -> Result<T, String>;
    fn to_string(&self, &T) -> Result<String, String>;
    fn decode_message(&self, &str) -> Result<Box<Self::M>, String>;
    fn encode_message(&self, message: &T, method: &str, id: u64) -> Result<String, String>;
}

pub trait CodecBase: Default {
    fn method(&self, s: &str) -> Result<String, String>;
    fn content_type(&self) -> Mime;
}

pub fn __decode_and_call<Request, Response, F, C>(ctx: &Context, codec: &C, body: &str, mut f: F, res: &mut TransportResponse)
    -> Result<(), ServeRequestError>
    where F: FnMut(&Context, <<C as Codec<Request>>::M as Message>::I) -> Response,
    C: Codec<Request> + Codec<Response>  {

    info!("message received: {}", body);
    let message = match <C as Codec<Request>>::decode_message(codec, body) {
        Ok(m) => m,
        Err(e) => return Err(ServeRequestError::InvalidBody(e))
    };
    info!("dispatching message to method {}", message.get_method());
    let res_raw = f(ctx, message.get_body().clone());
    let response_string = match <C as Codec<Response>>::encode_message(codec, &res_raw, message.get_method(), message.get_id()) {
        Ok(m) => Ok(m),
        Err(e) => Err(e)
    };

    match response_string {
        Ok(s) => {
            let _ = res.write_all(s.as_bytes());
            return Ok(());
        },
        Err(e) => Err(ServeRequestError::Custom(e))
    }
}