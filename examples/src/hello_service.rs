#![feature(custom_derive, plugin, specialization, custom_attribute)]
#![plugin(rpc_macros, serde_macros)]
#![rpc_service(JsonCodec = "::rpc::codec::json_codec::JsonCodec")]
#![allow(unused_imports)]

extern crate rpc;

use rpc::context::Context;

pub struct HelloService;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl HelloService {
    pub fn hello(&self, c: &Context, req: String) -> Result<i32, f32> {
        println!("from world: {}", req);
        Ok(42)
    }

    pub fn create_person(&self, c: &Context, req: Person) -> Result<bool, bool> {
        println!("from create_person: {:?}", req);
        Ok(true)
    }
}
