#![crate_type = "bin"]

extern crate protobuf;

mod unknown;

use protobuf::Message;
use protobuf::CodedInputStream;
use protobuf::ProtobufResult;

use unknown::Unknown;

use std::io::{self, Read};

fn main() {
    let mut _handle = io::stdin();
    let mut handle = _handle.lock();

    process_stream(&mut handle);
}

fn process_stream(stdin_stream: &mut Read) {
    let mut stream = CodedInputStream::new(stdin_stream);

    loop {
        match stream.eof() {
            Err(e) => panic!(e),
            Ok(true) => break,
            Ok(false) => {
				match stream.read_message::<Unknown>() {
                    Err(e) => println!("{}", e),
                    Ok(x) => println!("{:?}", x),
                }
			}
        }
    }
}
