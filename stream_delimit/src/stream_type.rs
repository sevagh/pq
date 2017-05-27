pub enum StreamType {
    Varint,
    Leb128,
    Kafka,
    Single,
}

pub fn string_to_stream_type(stream_type: &str) -> StreamType {
    match stream_type {
        "varint" => StreamType::Varint,
        "single" => StreamType::Single,
        "kafka" => StreamType::Kafka,
        "leb128" => StreamType::Leb128,
        _ => panic!("Unrecognized stream type"),
    }
}
