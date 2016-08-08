extern crate rpc;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate test_service;
extern crate http_transport;
// extern crate json_codec;
extern crate msgp_codec;

use std::thread;
use std::sync::Arc;


// use test_service::{PersonHandlerClient, PersonHandlerClientTrait, Person};
use test_service::{HelloClient, HelloData, HelloClientTrait};
use http_transport::HttpClient;
// use json_codec::JsonCodec;
use msgp_codec::MsgpCodec;
use rpc::Context;

type Client = HelloClient<HttpClient>;
// type Client = PersonHandlerClient<HttpClient>;

fn main() {
    let _ = env_logger::init();
    info!("calling server at address: 127.0.0.1:9999");
    let c = Arc::new(Client::new("http://127.0.0.1:9999/"));

    (0..1).map(|_| {
        let cc = c.clone();
        thread::spawn(move || {
            let mut ctx = Context::new();
            ctx.add_meta("X-Custom-Header", "yolo");
            let res = cc.hello::<MsgpCodec>(ctx, &HelloData{s: "helloword".to_string(), i: 42, f:54.});
            match res {
                Ok(v) => info!("received: {:?}", v),
                Err(e) => error!("client error: {}", e)
            }
        })
    }).collect::<Vec<_>>().into_iter().map(|j| j.join()).collect::<Vec<_>>();
}
