// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::marker::PhantomData;
use super::context::Context;

// clientbuilder

#[derive(Debug, Eq, PartialEq, Default)]
pub struct Client<Req, Res, Err> {
    endpoint: &'static str,
    req: PhantomData<Req>,
    res: PhantomData<Res>,
    err: PhantomData<Err>,
}

impl<Req, Res, Err> Client<Req, Res, Err>
    where Req: Default, Res: Default, Err: Default {
    pub fn call(&self, c: &Context, r: &Req) -> Result<Res, Err> {
        return Ok(Res::default());
    }
}

// #[allow(non_upper_case_globals)]
// pub const HelloClient: super::Client<CustomRequest, CustomResponse> = super::Client{endpoint: "hello", res: PhantomData, req: PhantomData};
