extern crate protobuf;

use protobuf::CodedInputStream;
use protobuf::MessageStatic;
use protobuf::ProtobufError;

use std::io::Read;
use std::io::StdinLock;

pub fn print_message<M: MessageStatic>(read: &mut StdinLock) {
    let stream = CodedInputStream::new(read);

    let message = match stream.eof() {
        Err(e) => panic!(e),
        Ok(true) => None,
        Ok(false) => Some(stream.read_message()),
    };

    println!("{:?}", message.unwrap());
}
