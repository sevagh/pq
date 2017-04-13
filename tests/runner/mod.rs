/* inspired by: https://github.com/BurntSushi/xsv/blob/master/tests/workdir.rs */

use std::env;
use std::path::PathBuf;
use std::io::Write;
use std::process::{Command, Output, Child, Stdio};

pub struct Runner {
    pub cmd: Command,
    pub tests_path: PathBuf,
    chld: Option<Child>,
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
        let mut cmd = Command::new(root.join("pq"));
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        Runner {
            cmd: cmd,
            tests_path: tests_path,
            chld: None,
        }
    }

    pub fn with_stdin(&mut self, contents: &[u8]) {
        self.cmd.stdin(Stdio::piped());
        let mut chld = self._spawn();
        chld.stdin
            .as_mut()
            .unwrap()
            .write_all(contents)
            .unwrap();
        self.chld = Some(chld);
    }

    fn _spawn(&mut self) -> Child {
        self.cmd.spawn().unwrap()
    }

    pub fn spawn(&mut self) {
        self.chld = Some(self._spawn());
    }

    pub fn output(&mut self) -> Output {
        println!("Bullshit");
        //self.chld.take().unwrap().wait_with_output().unwrap()
        match self.chld.take() {
            None => panic!("I fucking hate this language"),
            Some(x) => x.wait_with_output().unwrap(),
        }
    }
}
