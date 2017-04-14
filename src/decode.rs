use discovery::LoadedDescriptors;
use error::PqrsError;
use std::collections::BTreeMap;
use std::io::Write;
use std::path::PathBuf;
use std::result::Result;
use serde::{Deserialize, Serialize};
use serde_json::ser::Serializer;
use serde_protobuf::de::Deserializer;
use serde_protobuf::error::{Error, ErrorKind};
use serde_protobuf::descriptor::{Descriptors, MessageDescriptor};
use serde_value::Value;
use protobuf::CodedInputStream;
use protobuf::error::ProtobufError;

pub struct PqrsDecoder {
    pub loaded_descs: LoadedDescriptors,
    pub message_type: String,
    pub force: bool,
}

impl PqrsDecoder {
    pub fn new(msgtype: &Option<String>,
               fdsets: &[PathBuf],
               force: bool)
               -> Result<PqrsDecoder, PqrsError> {
        let mut load_mds = true;
        let loc_msg_type = match *msgtype {
            Some(ref x) => {
                load_mds = false;
                adjust_message_type(x)
            }
            None => String::from(""),
        };
        let loaded_descs = match LoadedDescriptors::from_fdsets(fdsets, load_mds) {
            Err(PqrsError::EmptyFdsetError()) => return Err(PqrsError::EmptyFdsetError()),
            Err(e) => return Err(e),
            Ok(x) => x,
        };
        Ok(PqrsDecoder {
               loaded_descs: loaded_descs,
               message_type: loc_msg_type,
               force: force,
           })
    }

    fn decode_message_(&self, data: &[u8], out: &mut Write) -> Result<(), PqrsError> {
        let mut serializer = Serializer::new(out);
        if !self.loaded_descs.message_descriptors.is_empty() {
            let contenders = discover_contenders(data,
                                                 &self.loaded_descs.descriptors,
                                                 &self.loaded_descs.message_descriptors);
            if contenders.is_empty() {
                return Err(PqrsError::NoContenderError());
            }
            let contender_max = contenders.iter().max_by_key(|x| x.len());
            contender_max.serialize(&mut serializer).unwrap();
        } else {
            let stream = CodedInputStream::from_bytes(data);
            let mut deserializer = Deserializer::for_named_message(&self.loaded_descs.descriptors,
                                                                   &self.message_type,
                                                                   stream)
                    .unwrap();
            match deser(&mut deserializer) {
                Ok(value) => value.serialize(&mut serializer).unwrap(),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    pub fn decode_message(&self, buf: &[u8], mut out: &mut Write) -> Result<(), PqrsError> {
        if !self.force {
            return self.decode_message_(buf, &mut out);
        }
        let mut offset = 0;
        let buflen = buf.len();
        while offset < buflen {
            for n in 0..offset + 1 {
                if self.decode_message_(&buf[n..(buflen - offset + n)], &mut out)
                       .is_ok() {
                    return Ok(());
                }
            }
            offset += 1;
        }
        Err(PqrsError::CouldNotDecodeError())
    }
}

fn adjust_message_type(m: &str) -> String {
    let mut loc_msg_type = String::new();
    let ch = m.chars().nth(0).unwrap();
    if ch != '.' {
        loc_msg_type.push('.');
    }
    loc_msg_type.push_str(m);
    loc_msg_type
}

fn discover_contenders(data: &[u8],
                       d: &Descriptors,
                       mds: &[MessageDescriptor])
                       -> Vec<BTreeMap<Value, Value>> {
    let mut contenders = Vec::new();
    for md in mds {
        let stream = CodedInputStream::from_bytes(data);
        let mut deserializer = Deserializer::new(d, md, stream);
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
    contenders
}

fn deser(deserializer: &mut Deserializer) -> Result<Value, PqrsError> {
    match Value::deserialize(deserializer) {
        Ok(x) => Ok(x),
        Err(Error(ErrorKind::Protobuf(ProtobufError::WireError(msg)), _)) => {
            if msg == "unexpected EOF" {
                return Err(PqrsError::EofError());
            }
            Err(PqrsError::ProtobufError(msg))
        }
        Err(e) => Err(PqrsError::SerdeError(String::from(e.description()))),
    }
}

pub fn decode_size(lead: &[u8], size: &mut u32) -> Result<(), PqrsError> {
    let mut is = CodedInputStream::from_bytes(lead);
    *size = match is.read_raw_varint32() {
        Ok(x) => x,
        _ => return Err(PqrsError::NoLeadingVarintError()),
    };
    Ok(())
}
