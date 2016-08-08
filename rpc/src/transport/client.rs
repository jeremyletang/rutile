// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use codec::{CodecBase, Codec};
use context::Context;

pub trait ClientTransport : Default {
    fn new(addr: String) -> Self;
    fn call<Request, Response, C>(&self, ctx: Context, endpoint: &str, req: &Request)
        -> Result<Response, String>
        where C: CodecBase + Codec<Request> + Codec<Response>,
        Request: Clone, Response: Clone;
}
