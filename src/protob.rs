use std::env;
use std::fs::{File, read_dir};
use std::io::Read;
use std::path::PathBuf;
use serde::de::Deserialize;
use serde_protobuf::descriptor::{Descriptors, MessageDescriptor};
use serde_protobuf::de::Deserializer;
use serde_value::Value;
use protobuf::{CodedInputStream, parse_from_reader};
use protobuf::descriptor::{FileDescriptorSet, FileDescriptorProto};

pub fn process_single(read: &mut Read) {
    let mut descriptors = Descriptors::new();
    let mut message_descriptors = Vec::new();

    for fdset_path in discover_fdsets() {
        let mut fdset_file = File::open(fdset_path.as_path()).unwrap();
        let fdset: FileDescriptorSet = parse_from_reader(&mut fdset_file).unwrap();
        descriptors.add_file_set_proto(&fdset);
        let fdset_protos: &[FileDescriptorProto] = fdset.get_file();
        for file_proto in fdset_protos.iter() {
            for message_proto in file_proto.get_message_type().iter() {
                message_descriptors.push(MessageDescriptor::from_proto(&fdset_path.to_string_lossy().into_owned().as_str(), message_proto));
            }
        }
    }

    descriptors.resolve_refs();
    let mut read_buf = Vec::new();
    read.read_to_end(&mut read_buf).unwrap();

    for md in message_descriptors {
        let stream = CodedInputStream::from_bytes(&read_buf);
        println!("ATTEMPT TO DESERIALIE WITH:\n\t{:?}\n", md.name());
        let mut deserializer = Deserializer::new(&descriptors, &md, stream);
        let value = Value::deserialize(&mut deserializer).unwrap();
        println!("RESULT:\n\t{:?}\n\n", value);
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
    fdset_files
}
