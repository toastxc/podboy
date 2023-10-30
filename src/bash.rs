use crate::result::Result;
use std::process::Command;

pub struct Bash;
impl Bash {
    pub fn cmd(command: impl Into<String>, attach: bool) -> Result<Option<String>> {
        if attach {
            Bash::spawn(command).map(|_| None)
        } else {
            Bash::exec(command).map(|a| a.into())
        }
    }
    pub fn spawn(args: impl Into<String>) -> Result<()> {
        Ok(Bash::common(args).spawn()?.wait()?).map(|_| ())
    }
    pub fn exec(args: impl Into<String>) -> Result<String> {
        let input = Bash::common(args).output()?;
        Ok(String::from_utf8({
            if !input.stdout.is_empty() {
                input.stdout
            } else {
                input.stderr
            }
        })?)
    }
    fn common(args: impl Into<String>) -> Command {
        let mut cmd = Command::new("bash");
        cmd.args(["-c", &args.into()]);
        cmd
    }

    pub fn username() -> Result<String> {
        let username = Self::cmd("echo $USER", false)?.unwrap();
        let username = username.trim();
        Ok(username.to_string())
    }
}

pub struct Container;

impl Container {
    pub fn path(container: impl Into<String>) -> Result<String> {
        Ok(format!(
            "{}{}.container.service",
            Self::dir()?,
            container.into()
        ))
    }
    pub fn dir() -> crate::result::Result<String> {
        Ok(format!("/home/{}/.config/systemd/user/", Bash::username()?,))
    }
    pub fn exists(container: impl Into<String>) -> Result<bool> {
        Ok(std::fs::read(Self::path(container)?).map(|_| ()).is_ok())
    }
    pub fn generate(container: impl Into<String>) -> Result<String> {
        Bash::exec(format!(
            "podman generate systemd {} --restart-policy always",
            container.into()
        ))
    }
    pub fn list() -> Result<String> {
        Bash::exec(format!(
            "ls {} | grep \".container.service\"",
            Container::dir()?
        ))
    }
}

pub struct Systemd;

impl Systemd {
    pub fn reload() -> Result<()> {
        Bash::exec("systemctl --user daemon-reload").map(|_| ())
    }
}
