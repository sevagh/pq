extern crate std;

use std::path::Path as Path;
use std::path::PathBuf as PathBuf;

pub fn discover_proto_files() -> Vec<PathBuf> {
    return discover_files("./proto", "proto");
}

pub fn discover_genrs_files() -> Vec<PathBuf> {
    return discover_files("./src/protobuf", "rs");
}

fn discover_files(path: &str, extension: &str) -> Vec<PathBuf> {
    let mut result_files = Vec::new();

    let paths = std::fs::read_dir(Path::new(path)).unwrap();

    for p in paths {
        let path = match p {
            Ok(p) => p.path(),
            Err(_) => continue,
        };

        match path.extension() {
            Some(x) => {
                if x != extension {
                    continue;
                }
            },
            None => continue,
        }

        result_files.push(path);
    }

    return result_files;
}
