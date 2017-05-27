use std::string::String;

pub enum StreamType {
    Varint,
    Leb128,
    Kafka,
    Single,
}

pub fn string_to_stream_type(stream_type: String) -> StreamType {
    match stream_type.as_str() {
        "varint" => StreamType::Varint,
        "single" => StreamType::Single,
        "kafka" => StreamType::Kafka,
        "leb128" => StreamType::Leb128,
        _ => panic!("Unrecognized stream type"),
    }
}
