#![feature(custom_derive, plugin, specialization, custom_attribute)]
#![plugin(rpc_macros, serde_macros)]

extern crate rpc;
extern crate json_codec;

mod hello;
mod person;

pub use hello::{Hello, HelloClient, HelloClientTrait};
pub use person::{Person, PersonHandler, PersonHandlerClient, PersonHandlerClientTrait};
