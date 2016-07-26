extern crate rpc;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate test_service;

use std::thread;
use std::sync::Arc;

use test_service::{PersonHandlerClient, Person};
use rpc::Context;
use rpc::json_codec::JsonCodec;

type Client = PersonHandlerClient<::rpc::http_transport::HttpClient>;

fn main() {
    let _ = env_logger::init();
    info!("calling server at address: 127.0.0.1:9999");
    let c = Arc::new(Client::new("http://127.0.0.1:9999/"));

    (0..10).map(|_| {
        let cc = c.clone();
        thread::spawn(move || {
            let res = cc.create::<JsonCodec>(&Context::new(),
                                                    &Person{name: "thug".to_string(), age: 42});
            match res {
                Ok(v) => info!("received: {:?}", v),
                Err(e) => error!("client error: {}", e)
            }
        })
    }).collect::<Vec<_>>().into_iter().map(|j| j.join()).collect::<Vec<_>>();
}
