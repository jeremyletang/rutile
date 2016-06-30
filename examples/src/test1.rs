#![feature(custom_derive, plugin, associated_consts, const_fn)]
#![plugin(rpc_macros, serde_macros)]
#![allow(unused_imports, unused_variables, dead_code)]

extern crate rpc;

use rpc::Service;
use std::marker::PhantomData;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CustomRequest {}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CustomResponse {}

impl rpc::JsonConvertible for CustomRequest {
    fn from_message(&mut self, m: &rpc::Message) {}
    fn to_message(&self, m: &mut rpc::Message) {}
}

impl rpc::JsonConvertible for CustomResponse {
    fn from_message(&mut self, m: &rpc::Message) {}
    fn to_message(&self, m: &mut rpc::Message) {}
}

#[derive(Default)]
pub struct Client<Req, Res> {
    endpoint: &'static str,
    req: PhantomData<Req>,
    res: PhantomData<Res>,
}

impl<Req, Res> Client<Req, Res> {
    pub fn call(&self) {
        println!("yolo");
    }
}

pub mod hello {
    use super::{CustomResponse, CustomRequest};
    use std::marker::PhantomData;

    pub struct Test<T> where T: Send + Sync + 'static{
        pub i: T,
    }

    #[rpc_service(JsonConvertible)]
    impl<T> Test<T> where T: Send + Sync + 'static {
        #[allow(non_upper_case_globals)]
        pub const HelloClient: super::Client<CustomRequest, CustomResponse> = super::Client{endpoint: "hello", res: PhantomData, req: PhantomData};
        pub fn hello(&self, req: CustomRequest, res: CustomResponse) -> ::rpc::RutileError {
            println!("from hello");
            None
        }
        pub fn world(&self, req: CustomRequest, res: CustomResponse) -> ::rpc::RutileError {
            println!("from world");
            None
        }
    }
}

fn main() {
    let t = hello::Test { i: 42 };
    println!("SERVICE NAME IS: {}", t.__rpc_service_name());
    for s in t.__rpc_list_methods() {
        println!("method: {}", s);
    }
    let mut message_hello = rpc::Message::default();
    let mut message_world = rpc::Message::default();
    message_hello.method = "test1.hello.Test.hello".to_string();
    message_world.method = "test1.hello.Test.world".to_string();
    t.__rpc_serve_request(rpc::make_empty_context(), message_hello);
    t.__rpc_serve_request(rpc::make_empty_context(), message_world);
    // println!("I: {}", hello::Test::<i32>::I);
}
