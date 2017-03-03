#![crate_type = "bin"]

include!(concat!(env!("OUT_DIR"), "/protob.rs"));

fn main() {
    println!("{}", message());
}
