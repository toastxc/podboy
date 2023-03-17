use std::{
    fmt,
    process::{Command, Output},
};

pub fn bash_exec(args: &str) -> Result<String, BashErr> {
    let mut cmd = bash_common(args);

    let res = cmd.output().unwrap();

    Ok(bash_out(res))
}

pub async fn bash_spawn(args: &str) {
    bash_common(args).spawn().unwrap().wait().unwrap();
}

fn bash_out(input: Output) -> String {
    let stdout = String::from_utf8(input.stdout).unwrap_or_default();

    let sterr = String::from_utf8(input.stderr).unwrap_or_default();

    format!("{}{}", stdout.trim(), sterr.trim())
}
fn bash_common(args: &str) -> Command {
    //let mut cmd = Command::new("bash");
    let mut cmd = Command::new("bash");
    cmd.args(["-c", args]);
    cmd
}
#[derive(Debug, Clone)]
pub struct BashErr(String);

impl fmt::Display for BashErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Something went wrong with shell\n{}", self.0)
    }
}
