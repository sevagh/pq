extern crate std;

use std::env;

pub fn discover_fdset_files() {
    let mut fdset_files = Vec::new();

    let mut home = match env::home_dir() {
        Some(path) => path,
        None => return,
    };
    home.push(".pq");

    let paths = std::fs::read_dir(home.as_path()).unwrap();

    for path in paths {
        match path.extension() {
            Some(extension) => fdset_files.push(path),
            None => None,
        }
    }
}
