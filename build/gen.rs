use std::path::PathBuf;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

pub fn gen_protob_includes(file_path: PathBuf, protob_path: &Path) {
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(protob_path)
        .unwrap();

    write!(f, "include!(\"./protobuf/{}\");\n", file_path.to_string_lossy().into_owned()).unwrap();
}

pub fn gen_protob_body(protob_path: &Path) {
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .open(protob_path)
        .unwrap();

    f.write_all(b" 
pub fn message() -> &'static str {
    \"Hello, Fuck!\"
}
").unwrap();
}
