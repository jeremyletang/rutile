// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use codec::{Codec, CodecBase};
use context::Context;
use transport::ClientTransport;

pub enum RpcError {
    HostUnreachable,
    Timeout,
}

pub struct Client<T> where T: ClientTransport {
    tc: T,
    url: String,
}

impl<T> Client<T> where T: ClientTransport {
    pub fn new(url: String) -> Client<T> {
        Client {
            tc: T::new(url.clone()),
            url: url,
        }
    }
    fn call<Request, Response, C>(&self, ctx: Context, endpoint: &str, req: &Request)
        -> Result<Response, String>
        where C: CodecBase + Codec<Request> + Codec<Response>,
        Request: Clone, Response: Clone {
        self.tc.call::<_, _, C>(ctx, endpoint, req)
    }
}
