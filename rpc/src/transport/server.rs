// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io::Write;
use std::net::SocketAddr;

use mime::Mime;
use handler::Handler;

pub trait TransportRequest {
    fn remote_addr(&self) -> SocketAddr;
    fn body(&self) -> &[u8];
    fn mime(&self) -> Mime;
    fn method(&self) -> &str;
}

pub trait TransportResponse: Write {}

pub trait TransportServer {
    fn handle(self) -> ListeningTransportHandler;
    fn using<H>(&mut self, h: H) where H: Handler;
    fn has_method(&self, &str) -> bool;
}

pub trait TransportListeningServer {
    fn close(&mut self) -> Result<(), ()>;
}

pub struct ListeningTransportHandler {
    listening_transport: Box<TransportListeningServer>
}

impl ListeningTransportHandler {
    pub fn new<T>(lt: T) -> ListeningTransportHandler where T: 'static + TransportListeningServer {
        ListeningTransportHandler {
            listening_transport: Box::new(lt)
        }
    }

    pub fn close(&mut self) -> Result<(), ()> {
        self.listening_transport.close()
    }
}
