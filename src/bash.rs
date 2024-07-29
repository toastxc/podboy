use crate::Result;
use std::process::Command;

pub struct Bash;
impl Bash {
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
        Ok(Self::exec("echo $USER")?.trim().to_string())
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
    pub fn dir() -> Result<String> {
        Ok(format!("/home/{}/.config/systemd/user/", Bash::username()?,))
    }

    pub fn generate(container: impl Into<String>) -> Result<String> {
        Bash::exec(format!(
            "podman generate systemd {} --restart-policy always",
            container.into()
        ))
    }
}
