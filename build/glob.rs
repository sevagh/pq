extern crate std;

use std::path::Path as Path;
use std::path::PathBuf as PathBuf;

pub fn discover_fdset_files() -> Vec<PathBuf> {
    let mut fdset_files = Vec::new();

    let paths = std::fs::read_dir(Path::new("./fdset")).unwrap();

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
