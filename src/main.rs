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
    /*
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("must have exactly one argument");
    }
    let ref pb_bin = args[1];
    write_file(&pb_bin);
    */
}
