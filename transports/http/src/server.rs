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
use std::io::{self, Read, Write};
use std::net::{SocketAddr, ToSocketAddrs};

use rpc::mime::Mime;
use rpc::{TransportServer, TransportListeningServer, ListeningTransportHandler,
    TransportRequest, TransportResponse, Handler, ServeRequestError, Context, CodecBase};

pub struct HttpServer {
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

impl HttpServer {
    pub fn new<To: ToSocketAddrs>(addr: To) -> Result<HttpServer, ()> {
        match HyperServer::http(addr) {
            Ok(s) => {
                Ok(HttpServer{
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

impl TransportListeningServer for Listening {
    fn close(&mut self) -> Result<(), ()> {
        let _ = self.listening.close();
        return Ok(())
    }
}

impl TransportServer for HttpServer {
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
    body: Vec<u8>,
    mime: Mime,
    method: String,
}

impl TransportRequest for HttpTransportRequest {
    fn remote_addr(&self) -> SocketAddr {
        self.remote_addr
    }

    fn body(&self) -> &[u8] {
        &*self.body
    }

    fn mime(&self) -> Mime {
        self.mime.clone()
    }

    fn method(&self) -> &str {
        &self.method
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
    codecs: Vec<Box<CodecBase>>
}

fn exists(codecs: &Vec<Box<CodecBase>>, codec: &Box<CodecBase>) -> bool {
    for c in codecs {
        if c.content_type() == codec.content_type() {
            return true;
        }
    }
    return false;
}

impl HttpHandler {
    pub fn new(handlers: Vec<Box<Handler>>) -> HttpHandler {
        let mut codecs = vec![];
        for h in &handlers {
            let new_codecs = h.codecs();
            for c in new_codecs {
                if !exists(&codecs, &c) {
                    codecs.push(c);
                }
            }
        }

        HttpHandler {
            handlers: handlers,
            codecs: codecs,
        }
    }
}

fn match_content_type(codecs: &Vec<Box<CodecBase>>, ct: &ContentType) -> bool {
    let &ContentType(ref mime) = ct;
    for c in codecs {
        if &c.content_type() == mime {
            return true;
        }
    }
    return false;
}

fn match_method(methods: &Vec<String>, method: &str) -> bool {
    for m in methods {
        if &*m == method {
            return true;
        }
    }
    return false;
}

impl HyperHandler for HttpHandler {
    fn handle<'a, 'k>(&'a self, mut req: Request<'a,'k>, mut res: Response<'a, Fresh>) {
        info!("new request from: {}", req.remote_addr);

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
        // check if the content-type is supported by this server
        if !match_content_type(&self.codecs, &ct) {
            return make_bad_request_error(&format!("rutile-rpc: unrecognized Content-Type, {}", ct), res);
        }

        // get the requrest body
        let mut body = Vec::new();
        let _ = req.read_to_end(&mut body);

        // read method from the body
        let ContentType(mime) = ct.clone();
        let mut method = String::new();

        for c in &self.codecs {
            if c.content_type() == mime {
                match c.method(&*body) {
                    Ok(m) => {
                        method = m;
                        break
                    },
                    Err(e) => return make_bad_request_error(&format!("rutile-rpc: unable to read method for content-type {}, {}", mime, e), res),
                }
            }
        }

        // make the HttpTransport{Request,Response}
        let mut tres = HttpTransportResponse{buf: vec![]};
        let mut treq = HttpTransportRequest {
            remote_addr: req.remote_addr,
            body: body,
            mime: mime,
            method: method.clone(),
        };

        // create context from headers
        let mut ctx = Context::new();
        for hv in req.headers.iter() {
            ctx.metas.insert(hv.name().to_string(), hv.value_string());
        }

        // then call the services to execute the method
        for h in &self.handlers {
            if match_content_type(&h.codecs(), &ct) && match_method(&h.methods(), &method) {
                match h.handle(ctx.clone(), &mut treq, &mut tres) {
                    Ok(_) => {
                        // write the response body
                        make_response(&tres.buf, res);
                        return
                    },
                    Err(e) => match e {
                        ServeRequestError::InvalidBody(err_string) => {
                            // the method match but the body was Invalid
                            // so we can return now
                            make_bad_request_error(
                                &format!("rutile-rpc: the body for the method {}, has an unexpected format: {}", method, err_string), res);
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
            } else {
                // nothing to do, this handler do not understand this codec or do not dispose this method
            }

        }

        // if we arrive here, the method was not found
        // just write an error
        make_bad_request_error(&format!("rutile-rpc: unrecognized method {} for Content-Type {}", method, ct), res)
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
