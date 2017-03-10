#![crate_type = "bin"]
extern crate protobuf;

use std::fs;
use std::env;
use std::path::PathBuf;

mod glob;
mod protob;
mod gen;

fn main() {
    let protob_path = PathBuf::from("./src/protob.rs");
    let protob_modfile_path = PathBuf::from("./src/proto/mod.rs");

    if protob_path.exists() {
        match fs::remove_file(protob_path.to_string_lossy().into_owned()) {
            Ok(x) => x,
            Err(_) => return,
        }
    }

    for genrs_file_to_remove in glob::discover_genrs_files() {
        match fs::remove_file(genrs_file_to_remove) {
            Ok(x) => x,
            Err(_) => return,
        }
    }

    let fdset_files = glob::discover_fdset_files();

    if fdset_files.is_empty() {
        panic!("No fdset files in ./fdset");
    }

    for file in &fdset_files {
        let file_str = file.to_string_lossy().into_owned();
        let written_files = protob::write_file(&file_str);

        println!("{:?} {:?}", protob_path, protob_modfile_path);
        gen::gen_protob_modfile(&protob_modfile_path, written_files);
    }

    gen::gen_protob_includes(&protob_path);
    gen::gen_protob_body(&protob_path);
}
