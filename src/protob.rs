use discovery::load_descriptors;
use error::PqrsError;
use std::io::Write;
use std::path::PathBuf;
use std::result::Result;
use serde::{Deserialize, Serialize};
use serde_json::ser::Serializer;
use serde_protobuf::de::Deserializer;
use serde_protobuf::error::{Error, ErrorKind};
use serde_value::Value;
use protobuf::CodedInputStream;
use protobuf::error::ProtobufError;

pub fn named_message(data: &[u8],
                     msg_type: &str,
                     out: &mut Write,
                     fdsets: Vec<PathBuf>)
                     -> Result<(), PqrsError> {
    let mut loc_msg_type = String::new();
    let ch = msg_type.chars().nth(0).unwrap();
    if ch != '.' {
        loc_msg_type.push('.');
    }
    loc_msg_type.push_str(msg_type);

    let loaded_descs = match load_descriptors(fdsets, false) {
        Err(PqrsError::EmptyFdsetError(msg)) => return Err(PqrsError::EmptyFdsetError(msg)),
        Err(e) => return Err(e),
        Ok(x) => x,
    };

    let stream = CodedInputStream::from_bytes(data);
    let mut deserializer =
        Deserializer::for_named_message(&loaded_descs.descriptors, &loc_msg_type, stream).unwrap();
    let mut serializer = Serializer::new(out);
    match deser(&mut deserializer) {
        Ok(value) => value.serialize(&mut serializer).unwrap(),
        Err(e) => return Err(e),
    }
    Ok(())
}

pub fn guess_message(data: &[u8], out: &mut Write, fdsets: Vec<PathBuf>) -> Result<(), PqrsError> {
    let loaded_descs = match load_descriptors(fdsets, true) {
        Err(PqrsError::EmptyFdsetError(msg)) => return Err(PqrsError::EmptyFdsetError(msg)),
        Err(e) => return Err(e),
        Ok(x) => x,
    };

    let mut serializer = Serializer::new(out);
    let mut contenders = Vec::new();
    for md in loaded_descs.message_descriptors {
        let stream = CodedInputStream::from_bytes(data);
        let mut deserializer = Deserializer::new(&loaded_descs.descriptors, &md, stream);
        match deser(&mut deserializer) {
            Ok(Value::Map(value)) => {
                let mut unknowns_found = 0;
                for v in value.values() {
                    match *v {
                        Value::Unit => unknowns_found += 1,
                        _ => continue,
                    }
                }
                if unknowns_found == 0 {
                    contenders.push(value);
                }
            }
            Ok(_) | Err(_) => continue,
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
        Ok(x) => Ok(x),
        Err(Error(ErrorKind::Protobuf(ProtobufError::WireError(msg)), _)) => {
            if msg == "unexpected EOF" {
                return Err(PqrsError::EofError(msg));
            }
            Err(PqrsError::ProtobufError(msg))
        }
        Err(e) => Err(PqrsError::SerdeError(String::from(e.description()))),
    }
}
