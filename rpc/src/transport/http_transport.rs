// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::net::SocketAddr;

use hyper::net::HttpListener;
use hyper::server::{Server, Listening, Request, Response, Fresh, Handler};

use service::Service;
use transport::{Transport, ListeningTransport, ListeningTransportHandler};

pub struct HttpTransport {
    server: Server<HttpListener>,
    services: Vec<Box<Service>>,
}

pub struct ListeningHttpTransport {
    listening: Listening
}

impl ListeningHttpTransport {
    pub fn new(l: Listening) -> ListeningHttpTransport {
        ListeningHttpTransport {
            listening: l
        }
    }
}

impl HttpTransport {
    pub fn new(addr: &SocketAddr) -> Result<HttpTransport, ()> {
        match Server::http(addr) {
            Ok(s) => {
                Ok(HttpTransport{
                    server: s,
                    services: vec![],
                })
            },
            Err(e) => {
                Err(())
            }
        }
    }
}

impl ListeningTransport for ListeningHttpTransport {
    fn close(&mut self) -> Result<(), ()> {
        self.listening.close();
        return Ok(())
    }
}

impl Transport for HttpTransport {
    fn handle(mut self) -> ListeningTransportHandler {
        let services: Vec<Box<Service>> = self.services.drain(..).collect();
        let listener = self.server.handle(HttpHandler::new(services)).unwrap();
        ListeningTransportHandler::new(ListeningHttpTransport::new(listener))
    }

    fn using<S>(&mut self, s: S) where S: Service {
        self.services.push(Box::new(s));
    }
}

pub struct HttpHandler {
    services: Vec<Box<Service>>,
}

impl HttpHandler {
    pub fn new(services: Vec<Box<Service>>) -> HttpHandler {
        HttpHandler {
            services: services
        }
    }
}

impl Handler for HttpHandler {
    fn handle<'a, 'k>(&'a self, req: Request<'a, 'k>, res: Response<'a, Fresh>) {

    }
}
