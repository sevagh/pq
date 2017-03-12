#![crate_type = "bin"]

extern crate protobuf;

include!("./protob.rs");

use std::io;
use std::io::Read;

fn main() {
    let mut buffer = Vec::new();
    let mut _handle = io::stdin();
    let mut handle = _handle.lock();

    match handle.read_to_end(&mut buffer) {
        Ok(_) => process_bytes(&buffer),
        Err(error) => println!("error: {}", error),
    }
}
