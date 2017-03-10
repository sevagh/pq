#![crate_type = "bin"]

extern crate protobuf;

include!("./protob.rs");

use std::io;

fn main() {
    let mut input = String::new();
    let mut handle = io::stdin().lock();
    process_bytes(&mut handle);
}
