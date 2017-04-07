use error::PqrsError;
use std::env;
use std::fs::{File, read_dir};
use std::path::PathBuf;
use std::result::Result;
use serde_protobuf::descriptor::{Descriptors, MessageDescriptor};
use protobuf::parse_from_reader;

pub struct LoadedDescriptors {
    pub descriptors: Descriptors,
    pub message_descriptors: Vec<MessageDescriptor>,
}

pub fn discover_fdsets(fdsetpath: Option<String>) -> Result<Vec<PathBuf>, PqrsError> {
    let mut fdset_files = Vec::new();

    let path = match fdsetpath {
        Some(x) => PathBuf::from(x),
        None => {
            let mut home = match env::home_dir() {
                Some(x) => x,
                None => return Err(PqrsError::InitError(String::from("Could not find $HOME"))),
            };
            home.push(".pq");
            home
        }
    };

    for p in read_dir(path.as_path()).unwrap() {
        let path = p.unwrap().path();
        if !path.is_dir() {
            fdset_files.push(path);
        }
    }
    if fdset_files.is_empty() {
        return Err(PqrsError::EmptyFdsetError(String::from("no files in fdset dir")));
    }
    Ok(fdset_files)
}

pub fn load_descriptors(fdsets: Vec<PathBuf>,
                    with_message_descriptors: bool)
                    -> Result<LoadedDescriptors, PqrsError> {
    let mut descriptors = Descriptors::new();
    let mut message_descriptors = Vec::new();

    let mut fdset_proto_load_ctr = 0;
    for fdset_path in fdsets {
        let mut fdset_file = File::open(fdset_path.as_path()).unwrap();
        let fdset_proto = match parse_from_reader(&mut fdset_file) {
            Err(_) => continue,
            Ok(x) => x,
        };
        fdset_proto_load_ctr += 1;
        descriptors.add_file_set_proto(&fdset_proto);
        if with_message_descriptors {
            for file_proto in fdset_proto.get_file().iter() {
                for message_proto in file_proto.get_message_type().iter() {
                    message_descriptors
                        .push(MessageDescriptor::from_proto(&fdset_path
                                                                 .to_string_lossy()
                                                                 .into_owned()
                                                                 .as_str(),
                                                            message_proto));
                }
            }
        }
    }

    if fdset_proto_load_ctr == 0 {
        return Err(PqrsError::EmptyFdsetError(String::from("no valid fdsets found")));
    }
    descriptors.resolve_refs();
    Ok(LoadedDescriptors { descriptors: descriptors, message_descriptors: message_descriptors })
}
