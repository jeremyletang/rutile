// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate log;
extern crate mime as mime_crate;
extern crate serde;
extern crate serde_json;
extern crate typemap;
mod context;
mod codec;
mod client;
mod transport;
mod server;
mod handler;

pub mod mime {
    pub use mime_crate::*;
}
pub use context::Context;
pub use codec::{CodecBase, Codec, Message, DefaultMessage, __decode_and_call};
pub use client::Client;
pub use server::{Listening, Server};
pub use transport::{ServerTransport, ListeningServerTransport, ListeningTransportHandler,
    ClientTransport, TransportResponse, TransportRequest};
pub use handler::{ServeRequestError, Handler};
