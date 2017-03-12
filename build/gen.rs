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

{}

pub fn process_bytes(data: &[u8]) {{
    let mut stream = CodedInputStream::from_bytes(data);

{}
}}
", format_msgdef_imports(msgdefs), format_mergefrom_calls(msgdefs)).unwrap();
}

fn format_msgdef_imports(msgdefs: &Vec<String>) -> String {
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
