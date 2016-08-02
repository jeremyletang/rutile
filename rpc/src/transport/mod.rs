// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io::{Read, Write};
use std::net::SocketAddr;

use handler::Handler;

pub mod http_transport;

pub trait TransportRequest {
    fn remote_addr(&self) -> SocketAddr;
    fn body(&self) -> &str;
}

pub trait TransportResponse: Write {}

pub trait Transport {
    fn handle(self) -> ListeningTransportHandler;
    fn using<H>(&mut self, h: H) where H: Handler;
    fn has_method(&self, &str) -> bool;
}

pub trait ListeningTransport {
    fn close(&mut self) -> Result<(), ()>;
}

pub struct ListeningTransportHandler {
    listening_transport: Box<ListeningTransport>
}

impl ListeningTransportHandler {
    pub fn new<T>(lt: T) -> ListeningTransportHandler where T: 'static + ListeningTransport {
        ListeningTransportHandler {
            listening_transport: Box::new(lt)
        }
    }

    pub fn close(&mut self) -> Result<(), ()> {
        self.listening_transport.close()
    }
}
