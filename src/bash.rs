use std::{fmt, process::Command};

pub fn bash_exec(args: &str) -> Result<String, BashErr> {
    let mut cmd = Command::new("bash");
    cmd.args(["-c", args]);

    let res = cmd.output().unwrap();

    match (res.status.success(), res) {
        (true, a) => Ok(String::from_utf8(a.stdout).unwrap().trim().to_string()),
        (false, e) => Err(BashErr(String::from_utf8(e.stderr).unwrap())),
    }
}

#[derive(Debug, Clone)]
pub struct BashErr(String);

impl fmt::Display for BashErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Something went wrong with shell\n{}", self.0)
    }
}
