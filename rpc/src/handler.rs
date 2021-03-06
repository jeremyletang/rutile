// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use mime::Mime;

use codec::CodecBase;
use context::Context;
use transport::{TransportRequest, TransportResponse};

pub enum ServeRequestError {
    InvalidBody(String),
    Custom(String),
}

pub trait Handler: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn service_name(&self) -> &'static str;
    fn methods(&self) -> Vec<String>;
    fn codecs(&self) -> Vec<Box<CodecBase>>;
    fn mimes(&self) -> Vec<Mime>;
    fn handle(&self, c: Context, req: &mut TransportRequest, res: &mut TransportResponse)
        -> Result<(), ServeRequestError>;
}
