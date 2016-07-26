// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::net::SocketAddr;

use handler::Handler;
use transport::http_transport::HttpTransport;
use transport::{Transport, ListeningTransportHandler};

pub struct Server<T = HttpTransport> where T: Transport {
    transport: T,
}

pub struct Listening {
    listening_transport: ListeningTransportHandler
}

impl Listening {
    pub fn close(&mut self) {
        let _ = self.listening_transport.close();
    }
}

impl<T> Server<T> where T: Transport {
    pub fn new(transport: T) -> Server<T> {
        Server {
            transport: transport,
        }
    }

    pub fn using<H>(&mut self, h: H) -> &mut Self where H: Handler {
        self.transport.using(h);
        return self;
    }

    pub fn run(self) -> Listening {
        Listening {
            listening_transport: self.transport.handle()
        }
    }

    pub fn has_method(&self, method: &str) -> bool {
        return self.transport.has_method(method)
    }
}

impl Server<HttpTransport> {
    pub fn http(addr: &SocketAddr) -> Result<Server<HttpTransport>, ()> {
        HttpTransport::new(addr)
            .map(Server::new)
    }
}
