use fdset_discovery::get_loaded_descriptors;
use error::*;
use std::io::Write;
use std::result::Result;
use serde::{Deserialize, Serialize};
use serde_json::ser::Serializer;
use serde_protobuf::de::Deserializer;
use serde_protobuf::error::{Error, ErrorKind};
use serde_protobuf::descriptor::{Descriptors, MessageDescriptor};
use serde_value::Value;
use protobuf::CodedInputStream;

pub struct PqrsDecoder {
    pub descriptors: Descriptors,
    pub message_descriptors: Vec<MessageDescriptor>,
    pub message_type: Option<String>,
}

impl PqrsDecoder {
    pub fn new(msgtype: Option<String>) -> Result<PqrsDecoder, PqrsError> {
        let loaded_descs = match get_loaded_descriptors() {
            Err(e) => return Err(PqrsError::FdsetDiscoveryError(e)),
            Ok(x) => x,
        };
        let mut descriptors = Descriptors::new();
        let mut message_descriptors = Vec::new();
        for (fdset_path, fdset) in loaded_descs {
            descriptors.add_file_set_proto(&fdset);
            match msgtype {
                None => {
                    message_descriptors.append(&mut fdset
                                                        .get_file()
                                                        .iter()
                                                        .flat_map(|x| {
                        x.get_message_type()
                            .iter()
                            .map(|y| {
                                     MessageDescriptor::from_proto(fdset_path
                                                                       .to_string_lossy()
                                                                       .into_owned()
                                                                       .as_str(),
                                                                   y)
                                 })
                            .collect::<Vec<_>>()
                    })
                                                        .collect::<Vec<_>>());
                }
                Some(_) => (),
            }
        }
        descriptors.resolve_refs();
        Ok(PqrsDecoder {
               descriptors: descriptors,
               message_descriptors: message_descriptors,
               message_type: msgtype,
           })
    }

    fn decode_message_(&self,
                       data: &[u8],
                       out: &mut Write,
                       is_tty: bool)
                       -> Result<(), DecodeError> {
        let value = match self.message_type {
            None => {
                match discover_contenders(data, &self.descriptors, &self.message_descriptors) {
                    Ok(value) => value,
                    Err(e) => return Err(e),
                }
            }
            Some(ref x) => {
                let stream = CodedInputStream::from_bytes(data);
                let mut deserializer = Deserializer::for_named_message(&self.descriptors,
                                                                       &(format!(".{}", x)),
                                                                       stream)
                        .unwrap();
                match deser(&mut deserializer) {
                    Ok(value) => value,
                    Err(e) => return Err(e),
                }
            }
        };
        if is_tty {
            match value.serialize(&mut Serializer::pretty(out)) {
                Ok(_) => Ok(()),
                Err(e) => Err(DecodeError::SerializeError(e)),
            }
        } else {
            match value.serialize(&mut Serializer::new(out)) {
                Ok(_) => Ok(()),
                Err(e) => Err(DecodeError::SerializeError(e)),
            }
        }
    }

    pub fn decode_message(&self,
                          buf: &[u8],
                          mut out: &mut Write,
                          is_tty: bool)
                          -> Result<(), PqrsError> {
        let mut offset = 0;
        let buflen = buf.len();
        while offset < buflen {
            for n in 0..offset + 1 {
                if self.decode_message_(&buf[n..(buflen - offset + n)], &mut out, is_tty)
                       .is_ok() {
                    return Ok(());
                }
            }
            offset += 1;
        }
        Err(PqrsError::DecodeError(DecodeError::NoSuccessfulAttempts))
    }
}

fn discover_contenders(data: &[u8],
                       d: &Descriptors,
                       mds: &[MessageDescriptor])
                       -> Result<Value, DecodeError> {
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
    if contenders.is_empty() {
        return Err(DecodeError::NoSuccessfulAttempts);
    }
    Ok(Value::Map(contenders.into_iter().max_by_key(|x| x.len()).unwrap()))
}

fn deser(deserializer: &mut Deserializer) -> Result<Value, DecodeError> {
    match Value::deserialize(deserializer) {
        Ok(x) => Ok(x),
        Err(Error(ErrorKind::Protobuf(e), _)) => Err(DecodeError::ProtobufError(e)),
        Err(e) => Err(DecodeError::SerdeProtobufError(e)),
    }
}
