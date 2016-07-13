// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use service::Service;
use transport::http_transport::HttpTransport;
use transport::Transport;

pub struct Server<T = HttpTransport> where T: Transport {
    services: Vec<Box<Service>>,
    transport: T,
}

impl<T> Server<T> where T: Transport {
    pub fn new(transport: T) -> Server<T> {
        Server {
            services: vec![],
            transport: transport,
        }
    }

    pub fn using<S>(&mut self, s: S) -> &mut Self where S: Service {
        self.services.push(Box::new(s));
        return self;
    }

    pub fn run() {

    }
}

impl Server<HttpTransport> {
    pub fn http(addr: &SocketAddr) -> Result<Server<HttpTransport>> {
        HttpTransport::new(addr)
    }
}
