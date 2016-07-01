#![feature(custom_derive, plugin, specialization)]
#![plugin(rpc_macros, serde_macros)]
#![allow(unused_imports, unused_variables, dead_code)]

extern crate rpc;

use rpc::context::{self, Context};
use rpc::service::Service;

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

#[rpc_service(JsonConvertible)]
pub mod test_service {
    use super::{CustomRequest, CustomResponse, Error};
    use rpc::context::Context;

    pub struct Test<T>
        where T: Send + Sync + 'static
    {
        pub i: T,
    }

    impl<T> Test<T>
        where T: Send + Sync + 'static
    {
        pub fn hello(&self, c: &Context, req: CustomRequest) -> Result<CustomResponse, Error> {
            println!("from hello");
            Ok(CustomResponse {})
        }
        pub fn world(&self, c: &Context, req: CustomRequest) -> Result<CustomResponse, Error> {
            println!("from world");
            Ok(CustomResponse {})
        }
    }

    impl Test<String> {
        pub fn hello_(&self, c: &Context, req: CustomRequest) -> Result<CustomResponse, Error> {
            println!("from hello spec in string");
            Ok(CustomResponse {})
        }
        pub fn world_(&self, c: &Context, req: CustomRequest) -> Result<CustomResponse, Error> {
            println!("from world spec int string");
            Ok(CustomResponse {})
        }
    }

}

fn main() {
    let t = test_service::Test { i: 42 };
    let t_spec = test_service::Test { i: "s".to_string() };
    println!("SERVICE NAME IS: {}", t.__rpc_service_name());
    for s in t.__rpc_list_methods() {
        println!("method: {}", s);
    }
    let mut message_hello = rpc::Message::default();
    let mut message_world = rpc::Message::default();
    message_hello.method = test_service::TEST1_TEST_SERVICE_TEST_HELLO.to_string();
    message_world.method = test_service::TEST1_TEST_SERVICE_TEST_WORLD.to_string();
    t.__rpc_serve_request(context::make_empty_context(), message_hello.clone());
    t.__rpc_serve_request(context::make_empty_context(), message_world.clone());
    message_hello.method = test_service::TEST1_TEST_SERVICE_TEST_HELLO_.to_string();
    message_world.method = test_service::TEST1_TEST_SERVICE_TEST_WORLD_.to_string();
    t_spec.__rpc_serve_request(context::make_empty_context(), message_hello);
    t_spec.__rpc_serve_request(context::make_empty_context(), message_world);
}
