extern crate std;

use std::env;
use std::path::PathBuf as PathBuf;

pub fn discover_fdset_files() -> Vec<PathBuf> {
    let mut fdset_files = Vec::new();

    let mut home = match env::home_dir() {
        Some(home) => home,
        None => return fdset_files,
    };

    home.push(".pq");

    let paths = std::fs::read_dir(home.as_path()).unwrap();

    for p in paths {
        let path = match p {
            Ok(p) => p.path(),
            Err(_) => continue,
        };

        match path.extension() {
            Some(x) => {
                if x != "fdset" {
                    continue;
                }
            },
            None => continue,
        }

        fdset_files.push(path);
    }

    return fdset_files;
}
