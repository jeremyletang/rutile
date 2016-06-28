
use std::collections::HashMap;

type Context = HashMap<String, String>;

struct Message {
    method: String,
    body: String,
    id: i64
}

pub trait Service {
    fn rpc_service_name(&self) -> &'static str;
    fn serve_rpc_request(&mut self, i: Context, ) -> bool;
}
