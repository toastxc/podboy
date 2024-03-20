use crate::bash::{Bash, Container, Systemd};
use crate::Result;
use std::io::ErrorKind;

pub mod ha;

pub fn string_add(input: impl Into<Vec<String>>) -> String {
    input.into().into_iter().fold(String::new(), |mut a, b| {
        a += &(b + " ");
        a
    })
}
pub fn run_dual_void(prefix: &str, args: Vec<String>) -> Result<()> {
    run_dual(prefix, args).map(|_| {})
}
pub fn run_dual(prefix: &str, args: Vec<String>) -> Result<Option<String>> {
    let mut args = args;
    // systemd prefix
    if prefix.contains("systemctl") {
        Systemd::reload()?;

        let mut arg = args.remove(args.len() - 1);
        arg += ".container";
        args.push(arg);
    };

    // podman edit command
    if args.contains(&String::from("edit")) {
        if !Container::exists(args.get(1).unwrap())? {
            return Err(std::io::Error::new(ErrorKind::NotFound, "Could not find file").into());
        }
        Bash::spawn(format!("vim {}", Container::path(args.get(1).unwrap())?))?;
        Systemd::reload()?;
        Ok(None)

        // everything else
    } else {
        // some systemd commands require attaching to terminal
        if args.contains(&"status".to_string()) {
            println!("spawned");
            Bash::spawn(format!("{prefix} {}", string_add(args)))?;
            Ok(None)
        } else {
            Bash::exec(format!("{prefix} {}", string_add(args))).map(|a| a.into())
        }
    }
}
