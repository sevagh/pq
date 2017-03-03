use std::path::PathBuf;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

pub fn gen_protob_includes(file_path: PathBuf, protob_path: &str) {
    let dest_path = Path::new(protob_path);

    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(&dest_path)
        .unwrap();

    write!(f, "include!(\"./protobuf/{}\");\n", file_path.to_string_lossy().into_owned()).unwrap();
}

pub fn gen_protob_body(protob_path: &str) {
    let dest_path = Path::new(protob_path);
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&dest_path)
        .unwrap();

    f.write_all(b" 
pub fn message() -> &'static str {
    \"Hello, Fuck!\"
}
").unwrap();
}
