#![crate_type = "bin"]
extern crate protobuf;

use std::fs;
use std::path::PathBuf;
use std::io::Error;

mod glob;
mod gen;

fn main() {
    let protob_path = PathBuf::from("./src/protob.rs");
    let protob_modfile_path = PathBuf::from("./src/schemata/mod.rs");

    delete_if_exists(&protob_path).unwrap();
    delete_if_exists(&protob_modfile_path).unwrap();

    gen::gen_protob_modfile(&protob_modfile_path, glob::discover_genrs_files());
    gen::gen_protob_includes(&protob_path);
    gen::gen_protob_body(&protob_path);
}

fn delete_if_exists(path: &PathBuf) -> Result<bool, Error> {
    if path.exists() {
        return match fs::remove_file(path.to_string_lossy().into_owned()) {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }
    Ok(false)
} 
