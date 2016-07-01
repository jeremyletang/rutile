// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use service::Service;
use transport::Transport;

pub struct Server {
    services: Vec<Box<Service>>,
    transport: Box<Transport>,
}

impl Server {
    pub fn using<S>(&mut self, s: S) -> &mut Self where S: Service {
        self.services.push(Box::new(s));
        return self;
    }
}
