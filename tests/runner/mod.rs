/* inspired by: https://github.com/BurntSushi/xsv/blob/master/tests/workdir.rs */

use std::env;
use std::path::PathBuf;
use std::process;

pub struct Runner {
    pub cmd: process::Command,
    pub tests_path: PathBuf,
}

impl Runner {
    pub fn new() -> Runner {
        let mut root = env::current_exe()
            .unwrap()
            .parent()
            .expect("executable's directory")
            .to_path_buf();
        if root.ends_with("deps") {
            root.pop();
        }
        let mut tests_path = root.parent().unwrap().parent().unwrap().to_path_buf();
        tests_path.push("tests");
        let cmd = process::Command::new(root.join("pq"));
        Runner {
            cmd: cmd,
            tests_path: tests_path,
        }
    }

    pub fn run(&mut self) -> process::Output {
        self.cmd.output().unwrap()
    }
}
