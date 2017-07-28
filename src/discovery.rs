extern crate protobuf;

use std::env;
use std::fs::{File, read_dir};
use std::path::PathBuf;
use protobuf::parse_from_reader;
use protobuf::descriptor::FileDescriptorSet;
use errors::*;

pub fn get_loaded_descriptors() -> Result<Vec<FileDescriptorSet>> {
    let (fdsets, fdset_path) = match discover_fdsets() {
        Ok((fdsets, fdsets_path)) => (fdsets, fdsets_path),
        Err(e) => return Err(e),
    };
    let mut descriptors: Vec<FileDescriptorSet> = Vec::new();

    for fdset_path in fdsets {
        let mut fdset_file = File::open(fdset_path.as_path()).chain_err(
            || "Couldn't open fdset file",
        )?;
        match parse_from_reader(&mut fdset_file) {
            Err(_) => continue,
            Ok(x) => descriptors.push(x),
        }
    }

    if descriptors.is_empty() {
        return Err(format!("no valid fdset files in {}", fdset_path).into());
    }
    Ok(descriptors)
}

fn discover_fdsets() -> Result<(Vec<PathBuf>, String)> {
    let mut fdset_files = Vec::new();

    let path = match env::var("FDSET_PATH") {
        Ok(x) => PathBuf::from(x),
        Err(_) => {
            let mut home = match env::home_dir() {
                Some(x) => x,
                None => return Err("$HOME is not defined".into()),
            };
            home.push(".pq");
            home
        }
    };

    let path_str = path.to_string_lossy().into_owned();

    match read_dir(path.as_path()) {
        Ok(paths) => {
            for p in paths {
                let path = p.expect("error iterating through paths").path();
                if !path.is_dir() {
                    fdset_files.push(path);
                }
            }
        }
        Err(_) => return Err(format!("Path {} not found", path_str).into()),
    }
    if fdset_files.is_empty() {
        return Err(format!("No valid fdset files in path {}", path_str).into());
    }
    Ok((fdset_files, path_str))
}
