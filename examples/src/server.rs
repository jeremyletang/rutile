extern crate rpc;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate test_service;
extern crate http_transport;

use rpc::Server;
use test_service::{Hello, PersonHandler};
use http_transport::HttpServerTransport;

fn main() {
    let _ = env_logger::init();
    let mut server = Server::new(HttpServerTransport::new(&"127.0.0.1:9999".parse().unwrap()).unwrap());
    server.using(Hello{});
    server.using(PersonHandler{});
    info!("starting server on 127.0.0.1:9999");
    server.run();
}
