// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use hyper::header::{ContentType, ContentLength, Allow};
use hyper::method::Method;
use hyper::net::HttpListener;
use hyper::server::Server as HyperServer;
use hyper::server::Listening as HyperListening;
use hyper::server::{Request, Response, Fresh};
use hyper::server::Handler as HyperHandler;
use rpc::mime::Mime;
use std::io::{self, Read, Write};
use std::net::SocketAddr;

use rpc::Context;
use rpc::{Handler, ServeRequestError};
use rpc::{ServerTransport, ListeningServerTransport, ListeningTransportHandler,
    TransportRequest, TransportResponse};


pub struct HttpServerTransport {
    server: HyperServer<HttpListener>,
    services: Vec<Box<Handler>>,
}

pub struct Listening {
    listening: HyperListening
}

impl Listening {
    pub fn new(l: HyperListening) -> Listening {
        Listening {
            listening: l
        }
    }
}

impl HttpServerTransport {
    pub fn new(addr: &SocketAddr) -> Result<HttpServerTransport, ()> {
        match HyperServer::http(addr) {
            Ok(s) => {
                Ok(HttpServerTransport{
                    server: s,
                    services: vec![],
                })
            },
            Err(_) => {
                Err(())
            }
        }
    }
}

impl ListeningServerTransport for Listening {
    fn close(&mut self) -> Result<(), ()> {
        let _ = self.listening.close();
        return Ok(())
    }
}

impl ServerTransport for HttpServerTransport {
    fn handle(mut self) -> ListeningTransportHandler {
        let services: Vec<Box<Handler>> = self.services.drain(..).collect();
        let listener = self.server.handle(HttpHandler::new(services)).unwrap();
        ListeningTransportHandler::new(Listening::new(listener))
    }

    fn using<H>(&mut self, h: H) where H: Handler {
        self.services.push(Box::new(h));
    }

    fn has_method(&self, method: &str) -> bool {
        for s in &self.services {
            match s.methods().iter().find(|ref x| **x == method) {
                Some(_) => return true,
                None => {}
            }
        }
        return false
    }
}

pub struct HttpTransportRequest {
    remote_addr: SocketAddr,
    body: String,
    mime: Mime,
}

impl TransportRequest for HttpTransportRequest {
    fn remote_addr(&self) -> SocketAddr {
        self.remote_addr
    }

    fn body(&self) -> &str {
        &*self.body
    }

    fn mime(&self) -> Mime {
        self.mime.clone()
    }
}

pub struct HttpTransportResponse {
    buf: Vec<u8>
}

impl Write for HttpTransportResponse {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.buf.extend_from_slice(data);
        Ok(data.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl TransportResponse for HttpTransportResponse {}


pub struct HttpHandler {
    handlers: Vec<Box<Handler>>,
    mimes: Vec<Mime>
}

impl HttpHandler {
    pub fn new(handlers: Vec<Box<Handler>>) -> HttpHandler {
        let mut mimes = vec![];
        for h in &handlers {
            mimes.append(&mut h.codecs());
        }
        mimes.dedup();
        HttpHandler {
            handlers: handlers,
            mimes: mimes,
        }
    }
}

impl HyperHandler for HttpHandler {
    fn handle<'a, 'k>(&'a self, mut req: Request<'a,'k>, mut res: Response<'a, Fresh>) {
        // add base headers
        make_base_headers(&mut res);
        // first check method
        if req.method != Method::Post {
            make_method_not_allowed_error(res, req.method);
            return
        }
        // then check content-type
        if !req.headers.has::<ContentType>() {
            make_bad_request_error("rutile-rpc: missing Content-Type header", res);
            return
        }
        // check is content-type is accepted by one of the services
        let ct = req.headers.get::<ContentType>().unwrap().clone();
        if !self.mimes.contains(&ct) {
            return make_bad_request_error(&format!("rutile-rpc: unrecognized Content-Type, {}", ct), res);
        }

        let mut body = String::new();
        let _ = req.read_to_string(&mut body);
        let ContentType(mime) = ct.clone();

        // make the HttpTransportRequest
        let mut transport_request = HttpTransportRequest {
            remote_addr: req.remote_addr,
            body: body,
            mime: mime,
        };
        let mut transport_response = HttpTransportResponse{buf: vec![]};

        // FIXME(JEREMY): we need in the future to fin a better way to handle method handling from this side
        let mut method_error = String::new();

        // then call the services to execute the method
        for h in &self.handlers {
            match h.handle(Context::new(), &mut transport_request, &mut transport_response) {
                Ok(_) => {
                    // write the response body
                    make_response(&transport_response.buf, res);
                    return
                },
                Err(e) => match e {
                    ServeRequestError::UnrecognizedMethod(method_err) => {
                        // continue for now, we may have over services that can handle this method
                        method_error = method_err;
                    },
                    ServeRequestError::NoMethodProvided(err_string) => {
                        make_bad_request_error(&format!("rutile-rpc: no method provided, {}", err_string), res);
                        return;
                    },
                    ServeRequestError::InvalidBody(err_string) => {
                        // the method match but the body was Invalid
                        // so we can return now
                        make_bad_request_error(
                            &format!("rutile-rpc: the body for the method {}, has an unexpected format: {}", "yolo", err_string), res);
                        return;
                    },
                    ServeRequestError::Custom(err_string) => {
                        // another kind of error occured,
                        // just write a nice message for the caller
                        make_bad_request_error(
                            &format!("rutile-rpc: something strange append ... this may help: {}", err_string), res);
                        return;
                    }
                }
            }
        }

        // if we arrive here, the method was not found
        // just write an error
        make_bad_request_error(&format!("rutile-rpc: unrecognized method {} for Content-Type {}", method_error, ct), res)
    }
}

fn make_response<'a>(body: &[u8], mut res: Response<'a, Fresh>) {
    res.headers_mut().set(ContentLength(body.len() as u64));
    *res.status_mut() = ::hyper::status::StatusCode::Ok;
    let mut res = res.start().unwrap();
    let _ = res.write_all(body);
}

fn make_base_headers<'a,>(res: &mut Response<'a, Fresh>) {
    res.headers_mut().set(Allow(vec![Method::Post]));
}

fn make_bad_request_error<'a,>(body: &str, mut res: Response<'a, Fresh>) {
    res.headers_mut().set(ContentLength(body.len() as u64));
    *res.status_mut() = ::hyper::status::StatusCode::BadRequest;
    let mut res = res.start().unwrap();
    let _ = res.write_all(body.as_bytes());
}

fn make_method_not_allowed_error<'a,>(mut res: Response<'a, Fresh>, method: Method) {
    let body = format!("rutile-rpc: POST method required, received {}", method);
    res.headers_mut().set(ContentLength(body.len() as u64));
    *res.status_mut() = ::hyper::status::StatusCode::MethodNotAllowed;
    let mut res = res.start().unwrap();
    res.write_all(body.as_bytes()).unwrap();
}
