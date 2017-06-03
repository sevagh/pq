use discovery::get_loaded_descriptors;
use error::*;
use std::io::Write;
use std::result::Result;
use serde::{Deserialize, Serialize};
use serde_json::ser::Serializer;
use serde_protobuf::de::Deserializer;
use serde_protobuf::error::{Error, ErrorKind};
use serde_protobuf::descriptor::Descriptors;
use serde_value::Value;
use protobuf::CodedInputStream;
use newline_pretty_formatter::NewlineFormatter;

pub struct PqrsDecoder<'a> {
    pub descriptors: Descriptors,
    pub message_type: &'a str,
}

impl<'a> PqrsDecoder<'a> {
    pub fn new(msgtype: &str) -> Result<PqrsDecoder, PqrsError> {
        let loaded_descs = match get_loaded_descriptors() {
            Err(e) => return Err(PqrsError::FdsetDiscoveryError(e)),
            Ok(x) => x,
        };
        let mut descriptors = Descriptors::new();
        for fdset in loaded_descs {
            descriptors.add_file_set_proto(&fdset);
        }
        descriptors.resolve_refs();
        Ok(PqrsDecoder {
               descriptors: descriptors,
               message_type: msgtype,
           })
    }

    pub fn decode_message(&self,
                          data: &[u8],
                          out: &mut Write,
                          is_tty: bool)
                          -> Result<(), DecodeError> {
        let stream = CodedInputStream::from_bytes(data);
        let mut deserializer = Deserializer::for_named_message(&self.descriptors,
                                                               &(format!(".{}",
                                                                         self.message_type)),
                                                               stream)
                .unwrap();
        let value = match Value::deserialize(&mut deserializer) {
            Ok(value) => value,
            Err(Error(ErrorKind::Protobuf(e), _)) => return Err(DecodeError::ProtobufError(e)),
            Err(e) => return Err(DecodeError::SerdeProtobufError(e)),
        };
        if is_tty {
            let formatter = NewlineFormatter::default();
            match value.serialize(&mut Serializer::with_formatter(out, formatter)) {
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
}
