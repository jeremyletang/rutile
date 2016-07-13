
use rpc::context::Context;

pub struct HelloService;

impl HelloService {
    pub fn hello(&self, c: &Context, req: String) -> Result<i32, f32> {
        println!("from world: {}", req);
        Ok(42)
    }
}
