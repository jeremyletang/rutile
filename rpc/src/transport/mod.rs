// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use service::Service;

pub mod http_transport;

pub trait Transport {
    fn handle(self) -> ListeningTransportHandler;
    fn using<S>(&mut self, s: S) where S: Service;
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
