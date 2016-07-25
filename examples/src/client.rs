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

use hello_service::HelloServiceClient;
use hello_service::Person;
use rpc::context::Context;
use rpc::codec::json_codec::JsonCodec;

type Client = HelloServiceClient<::rpc::transport::http_transport::HttpClient>;


fn main() {
    let _ = env_logger::init();
    info!("calling server at address: 127.0.0.1:9999");
    let c = Client::new("http://127.0.0.1:9999/");
    let res = c.create_person::<JsonCodec>(&Context::new(),
                                           &Person{name: "thug".to_string(), age: 42});
}
