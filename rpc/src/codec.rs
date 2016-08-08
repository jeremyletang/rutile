// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use mime::Mime;
use serde::{Serialize, Deserialize};

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
    fn extract_method_from_raw(&self, s: &[u8]) -> Result<String, String> {
        return self.method(s);
    }
    fn decode(&self, buf: &[u8]) -> Result<Box<Self::M>, String>;
    fn encode(&self, message: &T, method: &str, id: u64) -> Result<Vec<u8>, String>;
}

pub trait CodecBase: Default {
    fn method(&self, s: &[u8]) -> Result<String, String>;
    fn content_type(&self) -> Mime;
}

#[derive(Clone, Default, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct DefaultMessage<T> where T: Clone + Serialize + Deserialize {
    pub method: String,
    pub body: Option<T>,
    pub id: u64,
}

impl<T> Message for DefaultMessage<T> where T: Clone + Serialize + Deserialize {
    type I = T;
    fn get_method(&self) -> &str { &self.method }
    fn get_body(&self) -> &Self::I { &self.body.as_ref().unwrap() }
    fn get_id(&self) -> u64 { self.id }
    fn set_method(&mut self, method: &str) { self.method = method.to_string(); }
    fn set_body(&mut self, body: &Self::I) { self.body = Some(body.clone()); }
    fn set_id(&mut self, id: u64) { self.id = id; }
}

pub fn __decode_and_call<Request, Response, F, C>(ctx: &Context, codec: &C, body: &[u8], mut f: F, res: &mut TransportResponse)
    -> Result<(), ServeRequestError>
    where F: FnMut(&Context, <<C as Codec<Request>>::M as Message>::I) -> Response,
    C: Codec<Request> + Codec<Response>  {

    // info!("message received: {}", body);
    let message = match <C as Codec<Request>>::decode(codec, body) {
        Ok(m) => m,
        Err(e) => return Err(ServeRequestError::InvalidBody(e))
    };
    info!("dispatching message to method {}", message.get_method());
    let res_raw = f(ctx, message.get_body().clone());
    let response_string = match <C as Codec<Response>>::encode(codec, &res_raw, message.get_method(), message.get_id()) {
        Ok(m) => Ok(m),
        Err(e) => Err(e)
    };

    match response_string {
        Ok(s) => {
            let _ = res.write_all(&s);
            return Ok(());
        },
        Err(e) => Err(ServeRequestError::Custom(e))
    }
}
