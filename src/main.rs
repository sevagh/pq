#![crate_type = "bin"]

extern crate protobuf;

include!("./protob.rs");

use std::io::{self, Read};

fn main() {
    let mut _handle = io::stdin();
    let mut handle = _handle.lock();

    process_stream(&mut handle);
}
