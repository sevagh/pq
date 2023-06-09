use protobuf::descriptor::FileDescriptorSet;
use protobuf::parse_from_reader;
use std::{
    env,
    fs::{read_dir, File},
    path::{Path, PathBuf},
    process::Command,
};

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

    if let Some(mut x) = dirs::home_dir() {
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

pub fn compile_descriptors_from_proto(proto_file: &str) -> PathBuf {
    let fdset_path = env::temp_dir().join("tmp-pq.fdset");

    let mut cmd = Command::new(protoc());
    cmd.arg("--include_imports")
        .arg("--include_source_info")
        .arg("-o")
        .arg(&fdset_path)
        .arg(proto_file);

    cmd.arg("-I").arg(protoc_include());

    let output = cmd.output().expect("failed to execute protoc");
    if !output.status.success() {
        panic!("protoc failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    fdset_path
}

fn get_fdset_files_from_path(path: &Path) -> Vec<PathBuf> {
    let mut ret = vec![];
    if let Ok(paths) = read_dir(path) {
        for p in paths {
            let path = p.expect("error iterating through paths").path();
            if !path.is_dir() {
                ret.push(path);
            }
        }
    }
    ret
}

fn protoc() -> PathBuf {
    match env::var_os("PROTOC") {
        Some(protoc) => PathBuf::from(protoc),
        None => PathBuf::from(default_env!("PROTOC", "protoc")),
    }
}

fn protoc_include() -> PathBuf {
    match env::var_os("PROTOC_INCLUDE") {
        Some(include) => PathBuf::from(include),
        None => PathBuf::from(default_env!("PROTOC_INCLUDE", "")),
    }
}
