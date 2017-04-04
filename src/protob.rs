use std::env;
use std::fs::{File, read_dir};
use std::io::Write;
use std::path::PathBuf;
use std::result::Result;
use serde::{Deserialize, Serialize};
use serde_json::ser::Serializer;
use serde_protobuf::descriptor::{Descriptors, MessageDescriptor};
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
    match msg_type {
        "guess" => return guess_message(data, out),
        _ => return named_message(data, msg_type, out),
    }
}

fn named_message(data: &[u8], msg_type: &str, out: &mut Write) -> Result<(), PqrsError> {
    let mut loc_msg_type = String::new();
    let ch = msg_type.chars().nth(0).unwrap();
    if ch != '.' {
        loc_msg_type.push('.');
    }
    loc_msg_type.push_str(msg_type);

    let (descriptors, _) = get_descriptors(false);

    let stream = CodedInputStream::from_bytes(data);
    let mut deserializer = Deserializer::for_named_message(&descriptors, &loc_msg_type, stream).unwrap();
    let mut serializer = Serializer::new(out);
    match deser(&mut deserializer) {
        Ok(value) => value.serialize(&mut serializer).unwrap(),
        Err(e) => return Err(e),
    }
    Ok(())
}

fn guess_message(data: &[u8], out: &mut Write) -> Result<(), PqrsError> {
    let (descriptors, message_descriptors) = get_descriptors(true);
    
    let mut serializer = Serializer::new(out);
    let mut contenders = Vec::new();
    for md in message_descriptors {
        let stream = CodedInputStream::from_bytes(data);
        let mut deserializer = Deserializer::new(&descriptors, &md, stream);
        match deser(&mut deserializer) {
            Ok(Value::Map(value)) => {
                let mut unknowns_found = 0;
                for (_, v) in &value {
                    match v {
                        &Value::Unit => unknowns_found += 1,
                        _ => continue,
                    }
                }
                if unknowns_found == 0 {
                    contenders.push(value);
                }
            },
            Ok(_) => continue,
            Err(_) => continue,
        }
    }
    if !contenders.is_empty() {
        let contender_max = contenders.iter().max_by_key(|x| x.len());
        contender_max.serialize(&mut serializer).unwrap();
    }
    Ok(())
}

fn deser(deserializer: &mut Deserializer) -> Result<Value, PqrsError> {
    match Value::deserialize(deserializer) {
        Ok(x) => return Ok(x),
        Err(Error(ErrorKind::Protobuf(ProtobufError::WireError(msg)), _)) => {
            if msg == "unexpected EOF" {
                return Err(PqrsError::EofError(msg));
            }
            return Err(PqrsError::ProtobufError(msg));
        },
        Err(e) => return Err(PqrsError::SerdeError(String::from(e.description()))),
    };
}

fn get_descriptors(with_message_descriptors: bool) -> (Descriptors, Vec<MessageDescriptor>) {
    let mut descriptors = Descriptors::new();
    let mut message_descriptors = Vec::new();

    for fdset_path in discover_fdsets() {
        let mut fdset_file = File::open(fdset_path.as_path()).unwrap();
        let fdset_proto = parse_from_reader(&mut fdset_file).unwrap();
        descriptors.add_file_set_proto(&fdset_proto);
        if with_message_descriptors {
            for file_proto in fdset_proto.get_file().iter() {
                for message_proto in file_proto.get_message_type().iter() {
                    message_descriptors.push(MessageDescriptor::from_proto(&fdset_path.to_string_lossy().into_owned().as_str(), message_proto));
                }
            }
        }
    }
    descriptors.resolve_refs();
    (descriptors, message_descriptors)
}

fn discover_fdsets() -> Vec<PathBuf> {
    let mut home = env::home_dir().expect("Could not find $HOME");
    home.push(".pq");

    read_dir(home.as_path()).unwrap()
        .map(|x| {
            x.unwrap().path()
        })
        .collect::<Vec<_>>()
}
