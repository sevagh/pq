use std::path::PathBuf;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

pub fn gen_schemata_modfile(modfile_path: &Path, proto_path: &PathBuf) {
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(modfile_path)
        .unwrap();

    write!(f, "pub mod {};\n", proto_path.file_stem().unwrap().to_str().unwrap()).unwrap();
}

pub fn append_schemata_modfile(modfile_path: &Path, msgdefs: &Vec<String>) {
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(modfile_path)
        .unwrap();

    write!(f, "\n").unwrap();
    write!(f, "{}", format_msgdefs(msgdefs)).unwrap();
}

pub fn gen_protob_file(protob_path: &Path, msgdefs: &Vec<String>) {
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(protob_path)
        .unwrap();

    write!(&mut f, "mod schemata;

use protobuf::Message;
use protobuf::CodedInputStream;

use std::io::Read;
use std::io::StdinLock;

{}

pub fn process_bytes(read: &mut Read) {{
    let mut stream = CodedInputStream::new(read);

{}
}}
", format_msgdefs(msgdefs), format_mergefrom_calls(msgdefs)).unwrap();
}

fn format_msgdefs(msgdefs: &Vec<String>) -> String {
    let mut ret = Vec::new();

    for m in msgdefs {
        ret.push(format!("use schemata::{};", m));
    }
    ret.join("\n")
}

fn format_mergefrom_calls(msgdefs: &Vec<String>) -> String {
    let mut ret = Vec::new();

    for m in msgdefs {
        let split = m.split("::");
        let vec = split.collect::<Vec<&str>>();
        ret.push(format!("\tmatch {}::new().merge_from(&mut stream) {{
        Ok(x) => println!(\"{{:?}}\", x),
        Err(e) => panic!(e),
    }};", vec.last().unwrap()));
    }
    ret.join("\n")
}
