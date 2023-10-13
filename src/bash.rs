use crate::result::Result;
use std::process::Command;

pub struct Bash;
impl Bash {
    pub fn cmd(command: &str, attach: bool) -> Result<Option<String>> {
        if attach {
            Bash::spawn(command).map(|_| None)
        } else {
            Bash::exec(command).map(|a|Some(a))
        }
    }
    pub fn spawn(args: &str) -> Result<()> {
        Ok(Bash::common(args).spawn()?.wait()?).map(|_| ())
    }
    pub fn exec(args: &str) -> Result<String> {
        let input = Bash::common(args).output()?;
        Ok(String::from_utf8({
            if !input.stdout.is_empty() {
                input.stdout
            } else {
                input.stderr
            }
        })?)
    }
    fn common(args: &str) -> Command {
        let mut cmd = Command::new("bash");
        cmd.args(["-c", args]);
        cmd
    }

    pub fn username() -> Result<String> {
        let username = Self::cmd("echo $USER", false)?.unwrap();
        let username = username.trim();
        Ok(username.to_string())
    }
}
