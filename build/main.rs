#![crate_type = "bin"]
extern crate protobuf;

mod glob;
mod protob;

use std::path::Path;
use std::fs::File;
use std::io::Write;

fn main() {
    let fdset_files = glob::discover_fdset_files();

    for file in fdset_files {
        let file_str = file.to_string_lossy();
        protob::write_file(&file_str);
    }

    let dest_path = Path::new("./src/hello.rs");
    let mut f = File::create(&dest_path).unwrap();

    f.write_all(b" 
        pub fn message() -> &'static str {
            \"Hello, World!\"
        }
    ").unwrap();
}
