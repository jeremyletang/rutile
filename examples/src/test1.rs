#![feature(custom_derive, plugin, specialization)]
#![plugin(rpc_macros, serde_macros)]
#![allow(unused_imports, unused_variables, dead_code)]

extern crate rpc;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate env_logger;

use rpc::context::{self, Context};
use rpc::codec::Message;
use rpc::codec::json_codec::JsonMessage;
use rpc::codec::ContentTypeExtract;
use rpc::codec::json_codec::JsonCodec;
use rpc::service::Service;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CustomRequest {}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CustomResponse {}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Error {}


// impl rpc::codec::JsonConvertible for CustomRequest {
//     fn from_message(&mut self, m: &Message) {}
//     fn to_message(&self, m: &mut Message) {}
// }
//
// impl rpc::codec::JsonConvertible for Error {
//     fn from_message(&mut self, m: &Message) {}
//     fn to_message(&self, m: &mut Message) {}
// }
//
// impl rpc::codec::JsonConvertible for CustomResponse {
//     fn from_message(&mut self, m: &Message) {}
//     fn to_message(&self, m: &mut Message) {}
// }

#[rpc_service(JsonCodec = "::rpc::codec::json_codec::JsonCodec")]
pub mod test_service {
    use rpc::context::Context;
    use super::CustomRequest;

    pub struct Test<T>
        where T: Send + Sync + 'static
    {
        pub i: T,
    }

    // #[rpc_methods]
    impl<T> Test<T>
        where T: Send + Sync + 'static
    {
        pub fn hello(&self, c: &Context, req: CustomRequest) -> Result<String, f32> {
            // println!("from hello: {}", req);
            Ok("hello".to_string())
        }
        pub fn world(&self, c: &Context, req: String) -> Result<i32, f32> {
            println!("from world: {}", req);
            Ok(42)
        }
    }

    // impl Test<String> {
    //     pub fn hello_(&self, c: &Context, req: CustomRequest) -> Result<CustomResponse, Error> {
    //         println!("from hello spec in string");
    //         Ok(CustomResponse {})
    //     }
    //     pub fn world_(&self, c: &Context, req: CustomRequest) -> Result<CustomResponse, Error> {
    //         println!("from world spec int string");
    //         Ok(CustomResponse {})
    //     }
    // }

}

fn main() {
    ::rpc::codec::json_codec::JsonCodec::new().content_type();
    let _ = env_logger::init();
    let t = test_service::Test { i: 42 };
    let t_spec = test_service::Test { i: "s".to_string() };
    println!("SERVICE NAME IS: {}", t.__rpc_service_name());
    let mut message_hello = JsonMessage::<CustomRequest>::default();
    let mut message_world = JsonMessage::<String>::default();
    message_hello.set_method(test_service::TEST1_TEST_SERVICE_TEST_HELLO);
    // message_hello.set_body(&245);
    message_world.set_method(test_service::TEST1_TEST_SERVICE_TEST_WORLD);
    message_world.set_body(&"hello".to_string());

    t.__rpc_serve_request(Context::new(),
                          serde_json::to_string(&message_hello).unwrap());
    t.__rpc_serve_request(Context::new(),
                          serde_json::to_string(&message_world).unwrap());
    // message_hello.method = test_service::TEST1_TEST_SERVICE_TEST_HELLO_.to_string();
    // message_world.method = test_service::TEST1_TEST_SERVICE_TEST_WORLD_.to_string();
    // t_spec.__rpc_serve_request(context::make_empty_context(), message_hello);
    // t_spec.__rpc_serve_request(context::make_empty_context(), message_world);
}
