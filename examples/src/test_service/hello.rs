#![rpc_service(MsgpCodec = "::msgp_codec::MsgpCodec",
               JsonCodec = "::json_codec::JsonCodec")]

use rpc::Context;

pub struct Hello;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelloData {
    pub s: String,
    pub i: i32,
    pub f: f32,
}

#[rpc_methods]
impl Hello {
    pub fn hello(&self, ctx: ::rpc::Context , req: HelloData) -> Result<String, bool> {
        println!("from hello fn: {:?}", req);
        for (k, v) in ctx.metas {
            println!("key: {}, value: {}", k, v);
        }
        Ok("YEAH".to_string())
    }

    pub fn goodbye(&self, _: Context, req: bool) -> Result<String, Vec<String>> {
        println!("from create_person: {:?}", req);
        match req {
            true => Ok("true".to_string()),
            false => Err(vec!["false".to_string()])
        }
    }
}
