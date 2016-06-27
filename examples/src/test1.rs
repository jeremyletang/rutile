#![feature(custom_derive, plugin)]
#![plugin(rpc_macros)]

#[derive(Service)]
struct Test {
    i: i32
}

fn main(){}
