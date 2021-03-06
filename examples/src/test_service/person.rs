#![rpc_service(JsonCodec = "::json_codec::JsonCodec",
               MsgpCodec = "::msgp_codec::MsgpCodec")]

use rpc::Context;

pub struct PersonHandler;

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

#[rpc_methods]
impl PersonHandler {
    pub fn create(&self, _: Context, req: Person) -> Result<TestEnum, bool> {
        println!("from create: {:?}", req);
        Ok(TestEnum::First("thug life".to_string(), 32f32))
    }

    pub fn delete(&self, _: Context, req: Person) -> Result<TestEnum, bool> {
        println!("from create: {:?}", req);
        Ok(TestEnum::First("thug life".to_string(), 32f32))
    }

    pub fn update(&self, _: Context, req: Person) -> Result<TestEnum, bool> {
        println!("from create: {:?}", req);
        Ok(TestEnum::First("thug life".to_string(), 32f32))
    }

    pub fn list(&self, _: Context, req: Person) -> Result<TestEnum, bool> {
        println!("from create: {:?}", req);
        Ok(TestEnum::First("thug life".to_string(), 32f32))
    }
}
