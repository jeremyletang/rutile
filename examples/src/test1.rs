#![feature(custom_derive, plugin)]
#![plugin(rpc_macros, serde_macros)]
#![allow(unused_imports, unused_variables, dead_code)]

extern crate rpc;

use rpc::Service;

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomRequest {}
#[derive(Serialize, Deserialize, Debug)]
pub struct CustomResponse {}

impl rpc::JsonConvertible for CustomRequest {}
impl rpc::JsonConvertible for CustomResponse {}

pub mod hello {
    use super::{CustomResponse, CustomRequest};

    pub struct Test<T> {
        pub i: T,
    }

    #[rpc_service(JsonConvertible)]
    impl<T> Test<T> {
        pub fn hello(&mut self, req: CustomRequest, res: CustomResponse) -> ::rpc::RutileError {
            None
        }
        pub fn world(&mut self, req: CustomRequest, res: CustomResponse) -> ::rpc::RutileError {
            None
        }
    }

}

fn main() {
    let t = hello::Test { i: 42 };
    println!("SERVICE NAME IS: {}", t.__rpc_service_name());
    for s in t.__rpc_list_methods() {
        println!("method: {}", s);
    }
}
