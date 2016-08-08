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
extern crate rpc;

use std::net::SocketAddr;
use rpc::Server;

mod client;
mod server;

pub use server::HttpServer;
pub use client::HttpClient;

pub trait HttpTransport {
    fn http(addr: &SocketAddr) -> Self;
}

impl HttpTransport for Server<HttpServer> {
    fn http(addr: &SocketAddr) -> Server<HttpServer> {
        return Server::new(HttpServer::new(addr).unwrap());
    }
}
