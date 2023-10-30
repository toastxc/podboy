use crate::bash::Container;
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
            gen(args)
        }
        "gen" => gen(args),
        "rm" => rm(args),
        "ls" => {
            println!("{}", Container::list()?);
            Ok(())
        }
        _ => {
            error!(ErrorMsg::CLI_MISUSE);
        }
    }
}

fn rm(args: Vec<String>) -> Result<()> {
    let path = Container::path(args.get(1).unwrap())?;
    std::fs::read(&path)?;
    std::fs::remove_file(&path)?;
    Ok(())
}

// gen command
pub fn gen(args: Vec<String>) -> Result<()> {
    let path = Container::path(args.get(1).unwrap())?;

    if std::fs::read(&path).is_ok() {
        error!(ErrorMsg::FILE_EXISTS)
    };

    let container = Container::generate(args.get(1).unwrap())?;
    if container.contains("no such container") {
        error!(ErrorMsg::INVALID_CONTAINER)
    };
    let mut file = std::fs::File::create(&path)?;
    file.write_all(container.as_bytes())?;

    Ok(())
}
