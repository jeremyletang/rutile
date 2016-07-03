// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use context::Context;
use codec::Message;

pub trait Service: Send + Sync + 'static {
    fn __rpc_service_name(&self) -> &'static str;
    fn __rpc_list_methods(&self) -> Vec<String>;
    fn __rpc_list_supported_codecs(&self) -> Vec<String>;
    fn __rpc_serve_request(&self, c: Context, body: String) -> bool;
}
