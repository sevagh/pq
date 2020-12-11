use protobuf::descriptor::FileDescriptorSet;
use protobuf::CodedInputStream;
use serde::Serializer;

use serde_protobuf::de::Deserializer;
use serde_protobuf::descriptor::Descriptors;

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

    pub fn transcode_message<S: Serializer>(&self, data: &[u8], out: S) {
        let stream = CodedInputStream::from_bytes(data);
        let mut deserializer =
            Deserializer::for_named_message(&self.descriptors, self.message_type, stream)
                .expect("could not init deserializer");

        serde_transcode::transcode(&mut deserializer, out).unwrap();
    }
}
