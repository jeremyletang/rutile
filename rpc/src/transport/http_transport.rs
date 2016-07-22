// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use hyper::header::ContentLength;
use hyper::method::Method;
use hyper::net::HttpListener;
use hyper::server::{Server, Listening, Request, Response, Fresh, Handler};
use std::io::Write;
use std::net::SocketAddr;

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

    fn has_method(&self, method: &str) -> bool {
        for s in &self.services {
            match s.__rpc_list_methods().iter().find(|ref x| **x == method) {
                Some(_) => return true,
                None => {}
            }
        }
        return false
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
    fn handle<'a, 'k>(&'a self, req: Request<'a, 'k>, mut res: Response<'a, Fresh>) {
        if req.method != Method::Post {
            make_method_not_allowed_error(res, req.method);
            return
        }
        
    }
}

fn make_method_not_allowed_error<'a,>(mut res: Response<'a, Fresh>, method: Method) {
    let body = format!("rpc: POST method required, received {}", method);
    res.headers_mut().set(ContentLength(body.len() as u64));
    *res.status_mut() = ::hyper::status::StatusCode::MethodNotAllowed;
    let mut res = res.start().unwrap();
    res.write_all(body.as_bytes()).unwrap();
}
