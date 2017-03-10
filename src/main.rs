#![crate_type = "bin"]

extern crate protobuf;

include!("./protob.rs");

use std::io;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            process_bytes(input);
        }
        Err(error) => println!("error: {}", error),
    }
}
