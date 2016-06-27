#![feature(custom_derive, plugin)]
#![plugin(rpc_macros)]
#![allow(unused_imports, unused_variables, dead_code)]

pub trait JsonConvertible {}
impl JsonConvertible for i32 {}
impl JsonConvertible for f32 {}

pub trait ProtoConvertible {}
impl ProtoConvertible for i32 {}
impl ProtoConvertible for f32 {}

// #[rpc_service(JsonConvertible, ProtoConvertible)]
struct Test {
    i: i32,
}

#[rpc_service(JsonConvertible, ProtoConvertible)]
impl Test {
    fn hello(&mut self, i: i32, j: f32) {}
    fn world(&mut self, i: i32, j: f32) {}
}

fn main(){
    let t = Test{i: 42};
    println!("{}", t.list());
}
