use crate::bash::Bash;
use crate::result::{ErrorMsg, Result};
use crate::{error, HELP};
use std::io::Write;
pub fn run(args: Vec<String>) -> Result<()> {
    let arg = args.get(0).unwrap().as_str();

    if args.len() < 2 && arg != "ls" {
        println!("{HELP}");
        return Ok(());
    };
    match arg {
        "regen" => {
            rm(args.clone())?;
            gen(args)?;
        }
        "gen" => {
            gen(args)?;
        }
        "rm" => {
            rm(args)?;
        }
        "ls" => {
            println!(
                "{}",
                Bash::cmd(&format!("ls {}", container_dir()?), false)?.unwrap()
            )
        }
        _ => {
            error!(ErrorMsg::CLI_MISUSE);
        }
    }
    Ok(())
}

fn rm(args: Vec<String>) -> Result<()> {
    let path = container_path(args.get(1).unwrap())?;
    std::fs::read(&path)?;
    std::fs::remove_file(&path)?;
    Ok(())
}
fn container_path(container: &str) -> Result<String> {
    Ok(format!("{}/{container}.service", container_dir()?))
}
fn container_dir() -> Result<String> {
    Ok(format!("/home/{}/.config/systemd/user", Bash::username()?,))
}

pub fn gen(args: Vec<String>) -> Result<()> {
    // gen command
    let path = format!(
        "/home/{}/.config/systemd/user/{}.service",
        Bash::username()?,
        args.get(1).unwrap()
    );

    if std::fs::read(&path).is_ok() {
        error!(ErrorMsg::FILE_EXISTS)
    };

    let container = Bash::cmd(
        &format!(
            "podman generate systemd {} --restart-policy always",
            args.get(1).unwrap()
        ),
        false,
    )?
    .unwrap();

    if container.contains("no such container") {
        error!(ErrorMsg::INVALID_CONTAINER);
    };
    let mut file = std::fs::File::create(&path)?;
    file.write_all(container.as_bytes())?;

    Ok(())
}
