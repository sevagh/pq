use std::env;
use std::fs::{File, read_dir};
use std::io::Read;
use std::path::PathBuf;
use serde::de::Deserialize;
use serde_protobuf::descriptor::{Descriptors, MessageDescriptor};
use serde_protobuf::de::Deserializer;
use serde_value::Value;
use protobuf::{CodedInputStream, parse_from_reader};

pub fn process_single(read: &mut Read) {
    for mut fdset_path in discover_fdsets() {
        let fdset_file = File::open(fdset_path.as_path()).unwrap();
        let proto = parse_from_reader(&mut fdset_file).unwrap();
        let md = MessageDescriptor::from_proto(fdset_path, proto);
        let descriptors = Descriptors::from_proto(proto);
        let byte_is = CodedInputStream::new(read);

        let mut deserializer = Deserializer::new(&descriptors, md, byte_is).unwrap();
        let value = Value::deserialize(&mut deserializer).unwrap();
        println!("{:?}", value);
    }
}

pub fn process_stream(read: &mut Read) {
    let mut stream = CodedInputStream::new(read);

    loop {
        match stream.eof() {
            Err(e) => panic!(e),
            Ok(true) => break,
            Ok(false) => break, 
            //todo: actually do deserialization here
        }
    }
}

fn discover_fdsets() -> Vec<PathBuf> {
    let mut fdset_files = Vec::new();

    let mut home = env::home_dir().expect("Could not find $HOME");
    home.push(".pq");
    let paths = read_dir(home.as_path()).unwrap();

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
