#![crate_type = "bin"]
extern crate protobuf;

mod glob;
mod protob;
mod gen;

use std::path::Path;
use std::fs::File;
use std::io::Write;

fn main() {
    let fdset_files = glob::discover_fdset_files();

    for file in fdset_files {
        let file_str = file.to_string_lossy().into_owned();
        protob::write_file(&file_str);
        gen::gen_protob_fn(file);
    }
}
