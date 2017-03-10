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

    write!(f, "mod {};\n", proto_path.file_stem().unwrap().to_str().unwrap()).unwrap();
}

pub fn append_schemata_modfile(modfile_path: &Path, msgdefs: &Vec<String>) {
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(modfile_path)
        .unwrap();

    write!(f, "{}", format_msgdefs(msgdefs));
}

pub fn gen_protob_file(protob_path: &Path, msgdefs: &Vec<String>) {
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(protob_path)
        .unwrap();

    write!(&mut f, " 
mod schemata;

use protobuf::Message;
use protobuf::CodedInputStream;

use std::io::Read;
use std::io::StdinLock;

{}

pub fn process_bytes(read: &mut StdinLock) {{
    _process_bytes(read);
}}

fn _process_bytes(read: &mut StdinLock, msgdef: &Message) {{
    let stream = CodedInputStream::new(read);

    msgdef.merge_from(&mut stream).unwrap();
}}
", format_msgdefs(msgdefs)).unwrap();
}

fn format_msgdefs(msgdefs: &Vec<String>) -> String {
    let mut ret = Vec::new();

    for m in msgdefs {
        ret.push(format!("use schemata::{};", m));
    }
    ret.join("\n")
}
