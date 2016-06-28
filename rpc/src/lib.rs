
pub trait Service {
    fn rpc_service_name(&self) -> &'static str;
    fn serve_rpc_request(&mut self, i: i32) -> bool;
}
