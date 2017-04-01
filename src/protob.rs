use std::env;
use std::fs::{File, read_dir};
use std::io::Write;
use std::path::PathBuf;
use std::result::Result;
use serde::{Deserialize, Serialize};
use serde_json::ser::Serializer;
use serde_protobuf::descriptor::Descriptors;
use serde_protobuf::de::Deserializer;
use serde_protobuf::error::{Error, ErrorKind};
use serde_value::Value;
use protobuf::{CodedInputStream, parse_from_reader};
use protobuf::error::ProtobufError;

pub enum PqrsError {
    EofError(String),
    SerdeError(String),
    ProtobufError(String),
}

pub fn process_single(data: &[u8], msg_type: &str, out: &mut Write) -> Result<(), PqrsError> {
    let mut loc_msg_type = String::new();
    let ch = msg_type.chars().nth(0).unwrap();
    if ch != '.' {
        loc_msg_type.push('.');
    }
    loc_msg_type.push_str(msg_type);

    let mut descriptors = Descriptors::new();
    for fdset_path in discover_fdsets() {
        let mut fdset_file = File::open(fdset_path.as_path()).unwrap();
        let fdset_proto = parse_from_reader(&mut fdset_file).unwrap();
        descriptors.add_file_set_proto(&fdset_proto);
    }
    descriptors.resolve_refs();

    let stream = CodedInputStream::from_bytes(data);
    let mut deserializer = Deserializer::for_named_message(&descriptors, &loc_msg_type, stream).unwrap();
    let value = match Value::deserialize(&mut deserializer) {
        Ok(x) => x,
        Err(Error(ErrorKind::Protobuf(ProtobufError::WireError(msg)), _)) => {
            if msg == "unexpected EOF" {
                return Err(PqrsError::EofError(msg));
            }
            return Err(PqrsError::ProtobufError(msg));
        },
        Err(e) => return Err(PqrsError::SerdeError(String::from(e.description()))),
    };
    let mut serializer = Serializer::new(out);
    value.serialize(&mut serializer).unwrap();
    Ok(())
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
