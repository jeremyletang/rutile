#![rpc_service(JsonCodec = "::rpc::json_codec::JsonCodec")]
#![feature(custom_derive, plugin, specialization, custom_attribute)]
#![plugin(rpc_macros, serde_macros)]
// #![allow(unused_imports)]

extern crate rpc;

use rpc::Context;

pub struct HelloService;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestEnum {
    First(String, f32),
    Second(Vec<String>, String),
}

impl Default for TestEnum {
    fn default() -> TestEnum {
        return TestEnum::First("".to_string(), 42f32);
    }
}

#[rpc_methods]
impl HelloService {
    pub fn hello(&self, _: &::rpc::Context , req: String) -> String {
        println!("from world: {}", req);
        "hello".to_string()
    }

    pub fn create_person(&self, _: &Context, req: Person) -> TestEnum {
        println!("from create_person: {:?}", req);
        TestEnum::First("thug life".to_string(), 32f32)
    }
}
