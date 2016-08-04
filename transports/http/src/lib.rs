// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(custom_derive, plugin, integer_atomics)]

extern crate hyper;
#[macro_use]
extern crate log;
extern crate mime;
extern crate rpc;

mod client;
mod server;

pub use server::HttpServerTransport;
pub use client::HttpClientTransport;
