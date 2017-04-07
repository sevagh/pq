/* inspired by: https://github.com/BurntSushi/xsv/blob/master/tests/workdir.rs */

use std::env;
use std::fmt;
use std::path::PathBuf;
use std::process;
use std::str::FromStr;

pub struct Workdir {
    root: PathBuf,
    pub fdsets_path: PathBuf,
}

impl Workdir {
    pub fn new() -> Workdir {
        let mut root = env::current_exe().unwrap().parent().expect("executable's directory").to_path_buf();
        if root.ends_with("deps") {
            root.pop();
        }
        let mut fdsets_path = root.parent().unwrap().parent().unwrap().parent().unwrap().to_path_buf();
        fdsets_path.push("tests/fdsets");
        Workdir { root: root, fdsets_path: fdsets_path }
    }

    pub fn read_stdout(&self, cmd: &mut process::Command) -> String {
        let stdout: String = self.stdout(cmd);
        stdout
    }

    pub fn command(&self) -> process::Command {
        let mut cmd = process::Command::new(&self.pqrs_bin());
        cmd.arg(format!("--fdsets={}", self.fdsets_path.to_string_lossy().into_owned().as_str()));
        cmd
    }

    pub fn output(&self, cmd: &mut process::Command) -> process::Output {
        let o = cmd.output().unwrap();
        if !o.status.success() {
            panic!("\n\n===== {:?} =====\n\
                    command failed but expected success!\
                    \n\nstatus: {}\
                    \n\nstdout: {}\n\nstderr: {}\
                    \n\n=====\n",
                   cmd, o.status,
                   String::from_utf8_lossy(&o.stdout),
                   String::from_utf8_lossy(&o.stderr))
        }
        o
    }

    pub fn run(&self, cmd: &mut process::Command) {
        self.output(cmd);
    }

    pub fn stdout<T: FromStr>(&self, cmd: &mut process::Command) -> T {
        let o = self.output(cmd);
        let stdout = String::from_utf8_lossy(&o.stdout);
        stdout.trim_matches(&['\r', '\n'][..]).parse().ok().expect(
            &format!("Could not convert from string: '{}'", stdout))
    }

    pub fn assert_err(&self, cmd: &mut process::Command) {
        let o = cmd.output().unwrap();
        if o.status.success() {
            panic!("\n\n===== {:?} =====\n\
                    command succeeded but expected failure!\
                    \n\nstatus: {}\
                    \n\nstdout: {}\n\nstderr: {}\
                    \n\n=====\n",
                   cmd, o.status,
                   String::from_utf8_lossy(&o.stdout),
                   String::from_utf8_lossy(&o.stderr));
        }
    }

    pub fn pqrs_bin(&self) -> PathBuf {
        self.root.join("pq")
    }
}

impl fmt::Debug for Workdir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "path={}", self.root.display())
    }
}
