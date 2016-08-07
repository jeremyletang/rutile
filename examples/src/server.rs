extern crate rpc;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate test_service;
extern crate http_transport;

// use test_service::{Hello, PersonHandler};
use http_transport::HttpServer;
use http_transport::HttpServerTransport;
use rpc::Server;
use test_service::Hello;
// use test_service::{Hello, PersonHandler};

fn main() {
    let _ = env_logger::init();
    let mut server = Server::http(&"127.0.0.1:9999".parse().unwrap());
    server.using(Hello{});
    // server.using(PersonHandler{});
    info!("starting server on 127.0.0.1:9999");
    server.run();
}
