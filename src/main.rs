#![crate_type = "bin"]

#[macro_use]
extern crate clap;

#[macro_use]
extern crate default_env;

mod commands;
mod decode;
mod discovery;
mod formatter;

use crate::commands::*;

fn main() {
    let matches = clap_app!(
        @app (app_from_crate!())
        (@arg MSGTYPE: --msgtype +takes_value +global conflicts_with[CONVERT]
            "Sets protobuf message type")
        (@arg STREAM: --stream +takes_value "Enables stream + sets stream type")
        (@arg COUNT: --count +takes_value +global "Stop after count messages")
        (@arg CONVERT: --convert +takes_value +global "Convert to different stream type")
        (@arg EXTRA_FDSET_DIRS: --fdsetdir +takes_value +global +multiple
             "[repeatable] Specify dirs to load fdset files from")
        (@arg EXTRA_FDSET_FILES: --fdsetfile +takes_value +global +multiple
             "[repeatable] Specify an fdset file")
        (@arg EXTRA_PROTO_FILE: --protofile +takes_value +global
             "Specify a proto file")
        (@subcommand kafka =>
            (@arg TOPIC: +required "Sets the kafka topic")
            (@arg BROKERS: +required --brokers +takes_value "Comma-separated kafka brokers")
            (@arg FROMBEG: --beginning "Consume topic from beginning")
        )
    )
    .get_matches();

    let extra_fdset_dirs = match matches.values_of("EXTRA_FDSET_DIRS") {
        Some(dirs) => dirs.map(std::path::PathBuf::from).collect::<Vec<_>>(),
        None => vec![],
    };

    let extra_fdset_files = match matches.values_of("EXTRA_FDSET_FILES") {
        Some(files) => files.map(std::path::PathBuf::from).collect::<Vec<_>>(),
        None => vec![],
    };

    let extra_proto_file = matches.value_of("EXTRA_PROTO_FILE");

    let cmd = CommandRunner::new(extra_fdset_dirs, extra_fdset_files, extra_proto_file);

    match matches.subcommand() {
        ("kafka", Some(m)) => cmd.run_kafka(m),
        _ => cmd.run_byte(&matches),
    }
}
