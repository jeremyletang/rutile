// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate typemap;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate hyper;

pub mod context;
pub mod codec;
pub mod client;
pub mod transport;
pub mod server;
pub mod service;

pub mod ext_exports {
    pub use hyper::header::ContentType;
    pub use hyper::client::Client;
}
