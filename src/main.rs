#![crate_type = "bin"]

include!("./hello.rs");

fn main() {
    println!("{}", message());
}
