// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use codec::{Codec, CodecBase};
use context::Context;

pub trait Client : Default {
    fn new(addr: String) -> Self;
    fn call<Request, Success, Error, C>(&self, endpoint: &str, ctx: &Context, req: &Request)
        -> Result<Success, Error>
        where C: CodecBase + Codec<Request> + Codec<Success> + Codec<Error>,
        Request: Default, Success: Default, Error: Default;
}
