#![feature(custom_derive, plugin, specialization)]
#![plugin(rpc_macros, serde_macros)]
#![allow(unused_imports, unused_variables, dead_code)]

extern crate rpc;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hello_service;

use std::net::SocketAddr;
use rpc::server::Server;
use hello_service::HelloService;

fn main() {
    let _ = env_logger::init();
    let mut server = Server::http(&"127.0.0.1:9999".parse().unwrap()).unwrap();
    server.using(HelloService{});
    info!("starting server on 127.0.0.1:9999");
    server.run();
}
