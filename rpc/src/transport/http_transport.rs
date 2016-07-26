// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use hyper::client::Client as HyperClient;
use hyper::header::{ContentType, ContentLength, Allow};
use hyper::method::Method;
use hyper::net::HttpListener;
use hyper::server::{Server, Listening, Request, Response, Fresh};
use hyper::server::Handler as HyperHandler;
use std::io::{self, Read, Write};
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use codec::{CodecBase, Codec, Message};
use client::{Client, RpcError};
use context::Context;
use handler::{Handler, ServeRequestError};
use transport::{Transport, ListeningTransport,
    ListeningTransportHandler, TransportRequest, TransportResponse};

pub struct HttpTransport {
    server: Server<HttpListener>,
    services: Vec<Box<Handler>>,
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
            Err(_) => {
                Err(())
            }
        }
    }
}

impl ListeningTransport for ListeningHttpTransport {
    fn close(&mut self) -> Result<(), ()> {
        let _ = self.listening.close();
        return Ok(())
    }
}

impl Transport for HttpTransport {
    fn handle(mut self) -> ListeningTransportHandler {
        let services: Vec<Box<Handler>> = self.services.drain(..).collect();
        let listener = self.server.handle(HttpHandler::new(services)).unwrap();
        ListeningTransportHandler::new(ListeningHttpTransport::new(listener))
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

pub struct HttpTransportRequest<'a, 'k: 'a> {
    req: Request<'a, 'k>,
}

impl<'a, 'k> Read for HttpTransportRequest<'a, 'k> {
    fn read(&mut self, data: &mut [u8]) -> io::Result<usize> {
        self.req.read(data)
    }
}

impl<'a, 'k> TransportRequest for HttpTransportRequest<'a, 'k> {
    fn remote_addr(&self) -> SocketAddr {
        self.req.remote_addr
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
    content_types: Vec<ContentType>,
}

impl HttpHandler {
    pub fn new(handlers: Vec<Box<Handler>>) -> HttpHandler {
        let mut ct = vec![];
        for s in &handlers {
            ct.append(&mut s.codecs());
        }
        ct.dedup();
        HttpHandler {
            handlers: handlers,
            content_types: ct,
        }
    }
}

impl HyperHandler for HttpHandler {
    fn handle<'a, 'k>(&'a self, req: Request<'a,'k>, mut res: Response<'a, Fresh>) {
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
        if !self.content_types.contains(&ct) {
            return make_bad_request_error(&format!("rutile-rpc: unrecognized Content-Type, {}", ct), res);
        }

        // make the HttpTransportRequest
        let mut transport_request = HttpTransportRequest{req: req};
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
                        make_bad_request_error(&format!("rutile-rpc: {}", err_string), res);
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

#[derive(Clone)]
pub struct HttpClient {
    client: Arc<HyperClient>,
    url: String,
    current_id: Arc<AtomicU64>
}

impl Default for HttpClient {
    fn default() -> HttpClient {
        HttpClient {
            client: Arc::new(HyperClient::new()),
            url: "127.0.0.1:8000".to_string(),
            current_id: Arc::new(AtomicU64::new(1)),
        }
    }
}

impl Client for HttpClient {
    fn new(url: String) -> HttpClient {
        use std::time::Duration;
        let mut client = HyperClient::new();
        client.set_read_timeout(Some(Duration::new(2, 0)));
        client.set_write_timeout(Some(Duration::new(2,0)));
        HttpClient {
            client: Arc::new(client),
            url: url.clone(),
            current_id: Arc::new(AtomicU64::new(1)),
        }
    }

    fn call<Request, Response, C>(&self, endpoint: &str, ctx: &Context, req: &Request)
        -> Result<Response, String>
        where C: CodecBase + Codec<Request> + Codec<Response>,
        Request: Clone, Response: Clone {
        let id = self.current_id.clone().fetch_add(1, Ordering::SeqCst);

        let codec = C::default();
        let message = match <C as Codec<Request>>::encode_message(&codec, req, endpoint, id) {
            Ok(m) => m,
            Err(_) => unreachable!()
        };
        let cc = self.client.clone();
        let mut res = cc.post(&self.url)
            .header(codec.content_type())
            .body(&message)
            .send();

        match res {
            Ok(ref mut ok_res) => {
                let mut s = String::new();
                let _ = ok_res.read_to_string(&mut s);
                info!("response: {}", s);
                let concrete: &Response = match <C as Codec<Response>>::decode_message(&codec, &s) {
                    Ok(concrete) => unsafe {::std::mem::transmute(concrete.get_body())},
                    Err(e) => return Err(e)
                };
                return Ok((*concrete).clone());
            },
            Err(e) => {
                return Err(format!("{}", e));
            },
        }
    }
}
