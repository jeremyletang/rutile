#![rpc_service(JsonCodec = "::rpc::json_codec::JsonCodec")]

use rpc::Context;

pub struct Hello;

#[rpc_methods]
impl Hello {
    pub fn hello(&self, _: &::rpc::Context , req: String) -> Result<String, bool> {
        println!("from world: {}", req);
        Ok("hello".to_string())
    }

    pub fn goodbye(&self, _: &Context, req: bool) -> Result<String, Vec<String>> {
        println!("from create_person: {:?}", req);
        match req {
            true => Ok("true".to_string()),
            false => Err(vec!["false".to_string()])
        }
    }
}
