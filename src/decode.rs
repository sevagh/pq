use std::io::Write;
use serde::{Deserialize, Serialize};
use serde_json::ser::Serializer;
use serde_protobuf::de::Deserializer;
use serde_protobuf::descriptor::Descriptors;
use serde_value::Value;
use protobuf::CodedInputStream;
use protobuf::descriptor::FileDescriptorSet;
use formatter::CustomFormatter;
use errors::*;

pub struct PqrsDecoder<'a> {
    pub descriptors: Descriptors,
    pub message_type: &'a str,
}

impl<'a> PqrsDecoder<'a> {
    pub fn new(loaded_descs: Vec<FileDescriptorSet>, msgtype: &str) -> Result<PqrsDecoder> {
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

    pub fn decode_message(&self, data: &[u8], out: &mut Write, is_tty: bool) -> Result<()> {
        let stream = CodedInputStream::from_bytes(data);
        let mut deserializer = Deserializer::for_named_message(
            &self.descriptors,
            &(format!(".{}", self.message_type)),
            stream,
        ).expect("Couldn't initialize deserializer");
        let value = Value::deserialize(&mut deserializer).chain_err(
            || "Deser error",
        )?;
        Ok(value
            .serialize(&mut Serializer::with_formatter(
                out,
                CustomFormatter::new(is_tty),
            ))
            .chain_err(|| "Ser error")?)
    }
}
