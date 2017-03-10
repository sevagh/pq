#![crate_type = "bin"]
extern crate protobuf;

use std::fs;
use std::path::PathBuf;
use std::io::Error;

mod glob;
mod protob;
mod gen;

fn main() {
    let protob_path = PathBuf::from("./src/protob.rs");
    let protob_modfile_path = PathBuf::from("./src/proto/mod.rs");

    delete_if_exists(&protob_path).unwrap();
    delete_if_exists(&protob_modfile_path).unwrap();

    for genrs_file_to_remove in glob::discover_genrs_files() {
        fs::remove_file(genrs_file_to_remove).unwrap();
    }

    let fdset_files = glob::discover_fdset_files();

    if fdset_files.is_empty() {
        panic!("No fdset files in ./fdset");
    }

    for file in &fdset_files {
        let file_str = file.to_string_lossy().into_owned();
        let written_files = protob::write_file(&file_str);

        gen::gen_protob_modfile(&protob_modfile_path, written_files);
    }

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
