#![crate_type = "bin"]
extern crate protobuf;

use std::fs;
use std::path::PathBuf;
use std::io::{Error, BufRead, BufReader};

mod glob;
mod gen;

fn main() {
    let protob_path = PathBuf::from("./src/protob.rs");
    let schemata_modfile_path = PathBuf::from("./src/schemata/mod.rs");

    delete_if_exists(&protob_path).unwrap();
    delete_if_exists(&schemata_modfile_path).unwrap();

    let genrs_files = glob::discover_genrs_files();

    let mut deserialize_vec = Vec::new();

    for f in genrs_files {
        gen::gen_schemata_modfile(&schemata_modfile_path, &f);
        let file_stripped = f.clone();
        let filename = file_stripped.file_stem().unwrap().to_string_lossy().into_owned();

        let f = fs::File::open(f).unwrap();
        let f = BufReader::new(f);

        for line in f.lines() {
            let line_contents = line.unwrap();
            if line_contents.contains("impl ::protobuf::Message for") {
                let mut res: Vec<String> = line_contents.split_whitespace().map(|s| s.to_string()).collect();
                res.pop().unwrap();
                let last_word = res.pop().unwrap();
                deserialize_vec.push(format!("{}::{}", filename, last_word));
            }
        }
    }

    gen::append_schemata_modfile(&schemata_modfile_path, &deserialize_vec);
    gen::gen_protob_file(&protob_path, &deserialize_vec);
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
