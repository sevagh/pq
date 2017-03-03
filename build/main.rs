#![crate_type = "bin"]
extern crate protobuf;

use std::fs;
use std::env;
use std::path::Path;

mod glob;
mod protob;
mod gen;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let protob_path = Path::new(&out_dir).join("protob.rs");

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

    let proto_files = glob::discover_proto_files();

    if proto_files.is_empty() {
        panic!("No proto files in ./proto");
    }

    for file in proto_files {
        let file_str = file.to_string_lossy().into_owned();
        let written_files = protob::write_file(&file_str);

        for wf in written_files {
            gen::gen_protob_includes(wf, &protob_path);
        }
    }

    gen::gen_protob_body(&protob_path);
}
