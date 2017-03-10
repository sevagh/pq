use std::path::PathBuf;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

pub fn gen_protob_includes(protob_path: &Path) {
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(protob_path)
        .unwrap();

    write!(f, "mod schemata;\nmod decode;\n\nuse decode::print_message;\nuse std::io::StdinLock;\n").unwrap();
}

pub fn gen_protob_modfile(modfile_path: &Path, proto_paths: Vec<PathBuf>) {
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(modfile_path)
        .unwrap();

    for proto_f in proto_paths {
        write!(f, "mod {};\n", proto_f.file_stem().unwrap().to_str().unwrap()).unwrap();
    }
}

pub fn gen_protob_body(protob_path: &Path) {
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .open(protob_path)
        .unwrap();

    f.write_all(b" 
pub fn process_bytes(bytes: &mut StdinLock) {
    print_message(bytes);
}
").unwrap();
}
