use crate::formatter::CustomFormatter;
use protobuf::descriptor::FileDescriptorSet;
use protobuf::CodedInputStream;
use serde::{Deserialize, Serialize};
use serde_json::ser::Serializer;
use serde_protobuf::de::Deserializer;
use serde_protobuf::descriptor::Descriptors;
use serde_value::Value;
use std::io::Write;

pub struct PqDecoder<'a> {
    pub descriptors: Descriptors,
    pub message_type: &'a str,
}

impl<'a> PqDecoder<'a> {
    pub fn new(loaded_descs: Vec<FileDescriptorSet>, message_type: &str) -> PqDecoder<'_> {
        let mut descriptors = Descriptors::new();
        for fdset in loaded_descs {
            descriptors.add_file_set_proto(&fdset);
        }
        descriptors.resolve_refs();
        PqDecoder {
            descriptors,
            message_type,
        }
    }

    pub fn decode_message(&self, data: &[u8]) -> Value {
        let stream = CodedInputStream::from_bytes(data);
        let mut deserializer =
            Deserializer::for_named_message(&self.descriptors, self.message_type, stream)
                .expect("Couldn't initialize deserializer");
        match Value::deserialize(&mut deserializer) {
            Ok(x) => x,
            Err(e) => panic!("Couldn't decode message: {}", e),
        }
    }

    pub fn write_message(&self, v: Value, out: &mut dyn Write, formatter: &mut CustomFormatter) {
        if let Err(e) = v.serialize(&mut Serializer::with_formatter(out, formatter)) {
            panic!("Couldn't serialize message: {}", e);
        }
    }
}
