extern crate rpc;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hello_service;

use std::thread;
use std::sync::Arc;

use hello_service::HelloServiceClient;
use hello_service::Person;
use rpc::context::Context;
use rpc::codec::json_codec::JsonCodec;

type Client = HelloServiceClient<::rpc::transport::http_transport::HttpClient>;


fn main() {
    let _ = env_logger::init();
    info!("calling server at address: 127.0.0.1:9999");
    let c = Arc::new(Client::new("http://127.0.0.1:9999/"));
    let mut i = 0;

    while i != 10 {
        let cc = c.clone();
        thread::spawn(move || {
            let _ = cc.create_person::<JsonCodec>(&Context::new(),
                                                  &Person{name: "thug".to_string(), age: 42});

        });
        i += 1;
    }
    thread::sleep(::std::time::Duration::new(1, 0));
}
