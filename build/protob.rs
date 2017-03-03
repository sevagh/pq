extern crate protobuf;

use std::fs::*;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf as PathBuf;

use protobuf::parse_from_reader;
use protobuf::descriptor::*;
use protobuf::codegen::*;

pub fn write_file(bin: &str) -> Vec<PathBuf> {
    let mut rpaths: Vec<PathBuf> = Vec::new();

    let mut is = File::open(&Path::new(bin)).unwrap();
    let fds = parse_from_reader::<FileDescriptorSet>(&mut is as &mut Read).unwrap();

    let file_names: Vec<String> = fds.get_file().iter()
        .map(|f| f.get_name().to_string())
        .collect();

    let results = gen(fds.get_file(), &file_names);

    for r in &results {
        let mut rpath = PathBuf::from("./src/protobuf/");
        rpath.push(&r.name);

        let mut file_writer = File::create(&rpath).unwrap();
        file_writer.write(&r.content).unwrap();

        rpaths.push(PathBuf::from(&r.name));
    }

    return rpaths;
}
