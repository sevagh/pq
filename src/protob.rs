use protobuf::CodedInputStream;

use unknown::Unknown;

use std::io::Read;

pub fn process_single(read: &mut Read) {
    let mut buffer = Vec::new();
    read.read(&mut buffer).unwrap();

    let mut byte_is = CodedInputStream::from_bytes(&buffer);

    match byte_is.read_message::<Unknown>() {
        Err(e) => panic!(e),
        Ok(x) => println!("{:?}", x),
    }
}

pub fn process_stream(read: &mut Read) {
    let mut stream = CodedInputStream::new(read);

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
