use protobuf::descriptor::FileDescriptorSet;
use protobuf::parse_from_reader;
use std::env;
use std::fs::{read_dir, File};
use std::path::PathBuf;

pub fn get_loaded_descriptors(
    additional_fdset_dirs: Vec<PathBuf>,
    mut additional_fdset_files: Vec<PathBuf>,
) -> Vec<FileDescriptorSet> {
    let (mut fdsets, mut tested_things) = discover_fdsets(additional_fdset_dirs);
    fdsets.append(&mut additional_fdset_files);
    tested_things.append(
        &mut additional_fdset_files
            .iter()
            .map(|x| format!("File: {:?}", x))
            .collect::<Vec<_>>(),
    );

    let mut descriptors: Vec<FileDescriptorSet> = Vec::new();

    for fdset_path in fdsets {
        let mut fdset_file = match File::open(fdset_path.as_path()) {
            Ok(x) => x,
            Err(e) => panic!("Couldn't open fdset file: {}", e),
        };
        match parse_from_reader(&mut fdset_file) {
            Err(_) => continue,
            Ok(x) => descriptors.push(x),
        }
    }

    if descriptors.is_empty() {
        panic!("No valid fdset files found. Checked: {:#?}", tested_things);
    }
    descriptors
}

fn discover_fdsets(additional_fdset_dirs: Vec<PathBuf>) -> (Vec<PathBuf>, Vec<String>) {
    let mut tested_things = Vec::new();
    let mut fdset_files = Vec::new();

    if let Ok(x) = env::var("FDSET_PATH") {
        tested_things.push(format!("Directory: {:?}", x));
        let p = PathBuf::from(x);
        fdset_files.append(&mut get_fdset_files_from_path(&p));
    }

    if let Some(mut x) = env::home_dir() {
        x.push(".pq");
        tested_things.push(format!("Directory: {:?}", x));
        fdset_files.append(&mut get_fdset_files_from_path(&x));
    }

    for x in additional_fdset_dirs {
        tested_things.push(format!("Directory: {:?}", x));
        fdset_files.append(&mut get_fdset_files_from_path(&x));
    }

    let x = PathBuf::from("/etc/pq");
    tested_things.push(format!("Directory: {:?}", x));
    fdset_files.append(&mut get_fdset_files_from_path(&x));

    (fdset_files, tested_things)
}

fn get_fdset_files_from_path(path: &PathBuf) -> Vec<PathBuf> {
    let mut ret = vec![];
    if let Ok(paths) = read_dir(path.as_path()) {
        for p in paths {
            let path = p.expect("error iterating through paths").path();
            if !path.is_dir() {
                ret.push(path);
            }
        }
    }
    ret
}
