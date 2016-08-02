// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use codec::{Codec, CodecBase};
use context::Context;

pub enum RpcError {
    HostUnreachable,
    Timeout,
}

pub trait Client : Default {
    fn new(addr: String) -> Self;
    fn call<Request, Response, C>(&self, endpoint: &str, ctx: &Context, req: &Request)
        -> Result<Response, String>
        where C: CodecBase + Codec<Request> + Codec<Response>,
        Request: Clone, Response: Clone;
}
