#![feature(custom_derive, plugin, associated_consts)]
#![plugin(rpc_macros, serde_macros)]
#![allow(unused_imports, unused_variables, dead_code)]

extern crate rpc;

use rpc::{context, Service};
use rpc::context::Context;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CustomRequest {}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CustomResponse {}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Error {}


impl rpc::JsonConvertible for CustomRequest {
    fn from_message(&mut self, m: &rpc::Message) {}
    fn to_message(&self, m: &mut rpc::Message) {}
}

impl rpc::JsonConvertible for Error {
    fn from_message(&mut self, m: &rpc::Message) {}
    fn to_message(&self, m: &mut rpc::Message) {}
}

impl rpc::JsonConvertible for CustomResponse {
    fn from_message(&mut self, m: &rpc::Message) {}
    fn to_message(&self, m: &mut rpc::Message) {}
}

pub struct Test<T> where T: Send + Sync + 'static{
    pub i: T,
}

#[rpc_service(JsonConvertible)]
impl<T> Test<T> where T: Send + Sync + 'static {
    pub fn hello(&self, c: &Context, req: CustomRequest) -> Result<CustomResponse, Error>  {
        println!("from hello");
        Ok(CustomResponse{})
    }
    pub fn world(&self, c: &Context, req: CustomRequest) -> Result<CustomResponse, Error> {
        println!("from world");
        Ok(CustomResponse{})
    }
}

fn main() {
    let t = Test { i: 42 };
    println!("SERVICE NAME IS: {}", t.__rpc_service_name());
    for s in t.__rpc_list_methods() {
        println!("method: {}", s);
    }
    let mut message_hello = rpc::Message::default();
    let mut message_world = rpc::Message::default();
    message_hello.method = Test::<i32>::TEST1_TEST_HELLO.to_string();
    message_world.method = Test::<i32>::TEST1_TEST_WORLD.to_string();
    t.__rpc_serve_request(context::make_empty_context(), message_hello);
    t.__rpc_serve_request(context::make_empty_context(), message_world);
}
