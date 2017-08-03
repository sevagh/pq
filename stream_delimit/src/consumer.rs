use varint::decode_varint;
use std::io::Read;

pub struct StreamConsumer<'a> {
    consumer: &'a mut GenericConsumer,
}

impl<'a> StreamConsumer<'a> {
    pub fn new(consumer: &'a mut GenericConsumer) -> StreamConsumer<'a> {
        StreamConsumer { consumer }
    }
}

pub trait GenericConsumer {
    fn get_single_message(&mut self) -> Option<Vec<u8>>;
}

pub struct VarintConsumer<'a> {
    read: &'a mut Read,
}

pub struct SingleConsumer<'a> {
    read: &'a mut Read,
}

impl<'a> Iterator for StreamConsumer<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        self.consumer.get_single_message()
    }
}

impl<'a> VarintConsumer<'a> {
    pub fn new(read: &'a mut Read) -> VarintConsumer {
        VarintConsumer { read }
    }
}

impl<'a> SingleConsumer<'a> {
    pub fn new(read: &'a mut Read) -> SingleConsumer {
        SingleConsumer { read }
    }
}

impl<'a> GenericConsumer for VarintConsumer<'a> {
    fn get_single_message(&mut self) -> Option<Vec<u8>> {
        let ret: Option<Vec<u8>>;
        match decode_varint(self.read) {
            Ok(x) => {
                let mut msg_buf = vec![0; x as usize];
                match self.read.read_exact(&mut msg_buf) {
                    Ok(_) => (),
                    Err(_) => return None,
                }
                ret = Some(msg_buf);
            }
            Err(_) => ret = None,
        }
        ret
    }
}

impl<'a> GenericConsumer for SingleConsumer<'a> {
    fn get_single_message(&mut self) -> Option<Vec<u8>> {
        let ret: Option<Vec<u8>>;
        let mut buf = Vec::new();
        match self.read.read_to_end(&mut buf) {
            Ok(x) => {
                if x > 0 {
                    ret = Some(buf);
                } else {
                    ret = None
                }
            }
            Err(_) => ret = None,
        }
        ret
    }
}

impl<'a> GenericConsumer for StreamConsumer<'a> {
    fn get_single_message(&mut self) -> Option<Vec<u8>> {
        self.consumer.get_single_message()
    }
}
