use std::path::PathBuf;
use std::path::Path;
use std::fs::File;

pub fn gen_protob_fn(file_path: PathBuf) {
    let dest_path = Path::new("./src/protob.rs");
    let mut f = File::create(&dest_path).unwrap();

    write!(f, "include!(\"./protobuf/{}\");", "test.rs").unwrap();
    
    f.write_all(b" 

        pub fn message() -> &'static str {
            \"Hello, World!\"
        }
    ").unwrap();
}
