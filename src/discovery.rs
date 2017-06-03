extern crate protobuf;

use error::DiscoveryError;
use std::env;
use std::fs::{File, read_dir};
use std::path::PathBuf;
use std::result::Result;
use protobuf::parse_from_reader;
use protobuf::descriptor::FileDescriptorSet;

pub fn get_loaded_descriptors() -> Result<Vec<FileDescriptorSet>, DiscoveryError> {
    let (fdsets, fdset_path) = match discover_fdsets() {
        Ok((fdsets, fdsets_path)) => (fdsets, fdsets_path),
        Err(e) => return Err(e),
    };
    let mut descriptors: Vec<FileDescriptorSet> = Vec::new();

    for fdset_path in fdsets {
        let mut fdset_file = File::open(fdset_path.as_path()).unwrap();
        match parse_from_reader(&mut fdset_file) {
            Err(_) => continue,
            Ok(x) => descriptors.push(x),
        }
    }

    if descriptors.is_empty() {
        return Err(DiscoveryError::NoFiles(fdset_path));
    }
    Ok(descriptors)
}

fn discover_fdsets() -> Result<(Vec<PathBuf>, String), DiscoveryError> {
    let mut fdset_files = Vec::new();

    let path = match env::var("FDSET_PATH") {
        Ok(x) => PathBuf::from(x),
        Err(_) => {
            let mut home = match env::home_dir() {
                Some(x) => x,
                None => return Err(DiscoveryError::NoHome),
            };
            home.push(".pq");
            home
        }
    };

    let path_str = path.to_string_lossy().into_owned();

    match read_dir(path.as_path()) {
        Ok(paths) => {
            for p in paths {
                let path = p.unwrap().path();
                if !path.is_dir() {
                    fdset_files.push(path);
                }
            }
        }
        Err(_) => return Err(DiscoveryError::NoFdsetPath(path_str)),
    }
    if fdset_files.is_empty() {
        return Err(DiscoveryError::NoFiles(path_str));
    }
    Ok((fdset_files, path_str))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_simple() {
        assert_eq!("abc", "abc");
    }
}
