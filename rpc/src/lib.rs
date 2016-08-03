// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(custom_derive, plugin, integer_atomics)]
#![plugin(serde_macros)]

extern crate typemap;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate hyper;

mod context;
mod codec;
mod client;
mod transport;
mod server;
mod handler;

pub mod ext_exports {
    pub use hyper::header::ContentType;
    pub use hyper::client::Client;
}

pub use context::Context;
pub use codec::{CodecBase, Codec, json_codec, Message, __decode_and_call};
pub use client::Client;
pub use server::{Listening, Server};
pub use transport::{ServerTransport, ListeningServerTransport, ListeningTransportHandler,
    ClientTransport, TransportResponse, TransportRequest};
pub use handler::{ServeRequestError, Handler};
