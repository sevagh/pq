#![crate_type = "bin"]
extern crate protobuf;

mod glob;
mod protob;

fn main() {
    let fdset_files = glob::discover_fdset_files();

    for file in fdset_files {
        let file_str = file.to_string_lossy();
        protob::write_file(&file_str);
    }
}
