// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::net::SocketAddr;

use hyper::net::HttpListener;
use hyper::server::Server;

use super::Transport;

pub struct HttpTransport {
    server: Server<HttpListener>
}

impl HttpTransport {
    fn new(addr: &SocketAddr) -> Result<HttpTransport, > {
        match Server::http(addr) {
            Ok(s) => {
                Ok(HttpTransport{
                    server: s
                })
            },
            Err(e) => {

            }
        }
    }
}

impl Transport for HttpTransport {}
