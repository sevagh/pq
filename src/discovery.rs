extern crate protobuf;

use error::DiscoveryError;
use std::env;
use std::fs::{File, read_dir};
use std::path::PathBuf;
use std::result::Result;
use protobuf::parse_from_reader;
use protobuf::descriptor::{FileDescriptorSet, DescriptorProto};

pub fn get_descriptors_for_type(msgtype: &str) -> Result<DescriptorProto, DiscoveryError> {
    let fdsets = match discover_fdsets() {
        Ok(fdsets) => fdsets,
        Err(e) => return Err(e),
    };

    let msgtype_split = msgtype.rsplitn(2, '.').collect::<Vec<&str>>();
    for fdset_path in fdsets {
        let mut fdset_file = File::open(fdset_path.as_path()).unwrap();
        match parse_from_reader::<FileDescriptorSet>(&mut fdset_file) {
            Err(_) => continue,
            Ok(mut x) => {
                for mut fdproto in x.take_file().into_iter() {
                    if fdproto.get_package() == msgtype_split[1] {
                        for dproto in fdproto.take_message_type().into_iter() {
                            if dproto.get_name() == msgtype_split[0] {
                                return Ok(dproto);
                            }
                        }
                    }
                }
            }
        }
    }
    Err(DiscoveryError::NoMatchingFdProto)
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
        Err(_) => return Err(DiscoveryError::NoFdsetPath(path_str)),
    }
    if fdset_files.is_empty() {
        return Err(DiscoveryError::NoFiles(path_str));
    }
    Ok(fdset_files)
}
