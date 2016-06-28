#![feature(custom_derive, plugin)]
#![plugin(rpc_macros)]
#![allow(unused_imports, unused_variables, dead_code)]

extern crate rpc;

use rpc::Service;

pub trait JsonConvertible {}
impl JsonConvertible for i32 {}
impl JsonConvertible for f32 {}

pub trait ProtoConvertible {}
impl ProtoConvertible for i32 {}
impl ProtoConvertible for f32 {}

// #[rpc_service(JsonConvertible, ProtoConvertible)]
pub mod hello {
pub struct Test<T> {
    pub i: T,
}

#[rpc_service(JsonConvertible, ProtoConvertible)]
impl<T> Test<T> {
    pub fn hello(&mut self, i: i32, j: f32) {}
    pub fn world(&mut self, i: i32, j: f32) {}
}


}


fn main(){
    let mut t = hello::Test{i: 42};
    t.serve_rpc_request(42);
    t.serve_rpc_request(84);
}
