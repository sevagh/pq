use error::*;
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



impl LoadedDescriptors {

    pub fn new(with_message_descriptors: bool)
               -> Result<LoadedDescriptors, PqrsError> {
        let fdsets = match discover_fdsets() {
            Ok(fdsets) => fdsets,
            Err(e) => return Err(PqrsError::FdsetDiscoverError(e)),
        };
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
                            .push(MessageDescriptor::from_proto(fdset_path
                                                                    .to_string_lossy()
                                                                    .into_owned()
                                                                    .as_str(),
                                                                message_proto));
                    }
                }
            }
        }

        if fdset_proto_load_ctr == 0 {
            return Err(PqrsError::FdsetLoadError());
        }
        descriptors.resolve_refs();
        Ok(LoadedDescriptors {
               descriptors: descriptors,
               message_descriptors: message_descriptors,
           })
    }
}

fn discover_fdsets() -> Result<Vec<PathBuf>, DiscoveryError> {
    let mut fdset_files = Vec::new();

    let path = match env::var("FDSET_PATH") {
        Ok(x) => PathBuf::from(x),
        Err(_) => {
            let mut home = match env::home_dir() {
                Some(x) => x,
                None => return Err(DiscoveryError::NoHome),
            };
            home.push(".pq");
            home
        }
    };

    let path_str = path.to_string_lossy().into_owned();

    match read_dir(path.as_path()) {
        Ok(paths) => {
            for p in paths {
                let path = p.unwrap().path();
                if !path.is_dir() {
                    fdset_files.push(path);
                }
            }
        }
        Err(_) => return Err(DiscoveryError::NoFdsetPath(path_str))
    }
    if fdset_files.is_empty() {
        return Err(DiscoveryError::NoFiles(path_str))
    }
    Ok(fdset_files)
}
