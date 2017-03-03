#![crate_type = "bin"]
extern crate protobuf;

use std::fs;

mod glob;
mod protob;
mod gen;

static PROTOB_PATH : &'static str = "./src/protob.rs";

fn main() {
    match fs::remove_file(PROTOB_PATH) {
        Ok(x) => x,
        Err(_) => return,
    }

    for genrs_file_to_remove in glob::discover_genrs_files() {
        match fs::remove_file(genrs_file_to_remove) {
            Ok(x) => x,
            Err(_) => return,
        }
    }

    let fdset_files = glob::discover_fdset_files();

    for file in fdset_files {
        let file_str = file.to_string_lossy().into_owned();
        let written_files = protob::write_file(&file_str);

        for wf in written_files {
            gen::gen_protob_includes(wf, PROTOB_PATH);
        }
    }

    gen::gen_protob_body(PROTOB_PATH);
}
