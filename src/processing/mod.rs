use crate::bash::{Bash, Container, Systemd};
use crate::error;
use crate::result::ErrorMsg;
use result::Result;

pub mod ha;
pub mod result;
pub fn string_add(input: impl Into<Vec<String>>) -> String {
    input
        .into()
        .into_iter()
        .map(|item| format!("{item} "))
        .collect()
}
pub fn run_dual(cmd_type: bool, prefix: &str, args: Vec<String>) -> Result<Option<String>> {
    if prefix.contains("systemctl") {
        Systemd::reload()?;
    };

    if args.contains(&String::from("edit")) {
        if !Container::exists(args.get(1).unwrap())? {
            error!(ErrorMsg::FILE_NOT_FOUND);
        }
        Bash::spawn(format!("vim {}", Container::path(args.get(1).unwrap())?))?;
        Systemd::reload()?;
        Ok(None)
    } else {
        Bash::cmd(format!("{prefix} {}", string_add(args)), cmd_type)
    }
}
