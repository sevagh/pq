#![crate_type = "bin"]

#[macro_use]
extern crate clap;
extern crate libc;
extern crate protobuf;
extern crate serde;
extern crate serde_protobuf;
extern crate serde_value;
extern crate serde_json;
extern crate stream_delimit;
#[macro_use]
extern crate error_chain;

mod discovery;
mod newline_pretty_formatter;
mod decode;
mod commands;

mod errors {
    error_chain!{
        foreign_links {
            StreamDelimit(::stream_delimit::error::StreamDelimitError);
        }
    }
}

use errors::*;
use commands::*;

quick_main!(|| -> Result<i32> {
    let matches = clap_app!(
        @app (app_from_crate!())
        (@arg MSGTYPE: --msgtype +takes_value +global conflicts_with[CONVERT]
            "Sets protobuf message type")
        (@arg STREAM: --stream +takes_value "Enables stream + sets stream type")
        (@arg COUNT: --count +takes_value +global "Stop after count messages")
        (@arg CONVERT: --convert +takes_value +global "Convert to different stream type")
        (@subcommand kafka =>
            (@arg TOPIC: +required "Sets the kafka topic")
            (@arg BROKERS: +required --brokers +takes_value "Comma-separated kafka brokers")
            (@arg FROMBEG: --beginning "Consume topic from beginning")
        )
    ).get_matches();

    match matches.subcommand() {
        ("kafka", Some(m)) => run_kafka(m)?,
        _ => run_byte(&matches)?,
    }
    Ok(0)
});
