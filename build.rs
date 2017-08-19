extern crate capnpc;

use std::process::Command;
use std::env;

fn main() {
    capnpc::CompilerCommand::new()
        .file("tests/simple.capnp")
        .run()
        .expect("schema compiler command");
}