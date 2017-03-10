#![crate_type = "bin"]

extern crate protobuf;

include!("./protob.rs");

fn main() {
    println!("{}", message());
}
