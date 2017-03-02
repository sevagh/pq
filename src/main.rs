#![crate_type = "bin"]

include!("./protob.rs");

fn main() {
    println!("{}", message());
}
