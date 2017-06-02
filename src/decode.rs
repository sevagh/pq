use discovery::get_descriptors_for_type;
use error::*;
use std::io::Write;
use std::result::Result;
use protobuf::Message;
use protobuf::descriptor::DescriptorProto;

pub struct PqrsDecoder {
    dproto: DescriptorProto,
}

impl PqrsDecoder {
    pub fn new(msgtype: &str) -> Result<PqrsDecoder, PqrsError> {
        match get_descriptors_for_type(msgtype) {
            Err(e) => return Err(PqrsError::FdsetDiscoveryError(e)),
            Ok(x) => {
                Ok(PqrsDecoder {
                    dproto: x,
                })
            },
        }
    }

    pub fn decode_message(&mut self,
                          data: &[u8],
                          out: &mut Write,
                          is_tty: bool)
                          -> Result<(), DecodeError> {
        match self.dproto.merge_from_bytes(data) {
            Ok(x) => writeln!(out, "{:?}", x).unwrap(),
            Err(e) => return Err(DecodeError::ProtobufError(e)),
        }
        Ok(())
    }
}
