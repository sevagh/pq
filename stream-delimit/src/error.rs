#![allow(unused_doc_comment)]

error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }

    links {
        Kafka(::kafka::error::Error, ::kafka::error::ErrorKind) #[cfg(feature="with_kafka")];
    }

    errors {
        #[cfg(feature="with_kafka")]
        /// Error connecting to Kafka
        KafkaInitializeError(e: ::kafka::Error) {
            description("couldn't initialize kafka consumer")
            display("couldn't initialize kafka consumer: {}", e)
        }

        /// Error decoding a varint|leb128 stream
        VarintDecodeError(e: ::std::io::Error) {
            description("couldn't decode leading varint")
            display("couldn't decode leading varint: '{}'", e)
        }

        /// Unrecognized stream type
        InvalidStreamTypeError(t: String) {
            description("invalid stream type")
            display("invalid stream type: {} ((only support single, leb128, varint)", t)
        }

        /// Too large to be a valid varint|leb128
        VarintDecodeMaxBytesError {
            description("exceeded max attempts to decode leading varint")
            display("exceeded max attempts to decode leading varint")
        }
    }
}
