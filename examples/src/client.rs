#![feature(custom_derive, plugin, specialization)]
#![plugin(serde_macros)]
#![allow(unused_imports, unused_variables, dead_code)]

extern crate rpc;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hello_service;

use std::net::SocketAddr;

fn main() {
    let _ = env_logger::init();
    info!("calling server at address: 127.0.0.1:9999");
}
