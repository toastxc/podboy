use crate::bash::Container;
use crate::{Result, HELP};
use std::io::{ErrorKind, Write};

pub fn run(args: Vec<String>) -> Result<()> {
    let arg = args.first().unwrap().as_str();

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
            // error!(ErrorMsg::CLI_MISUSE);
            Err(std::io::Error::new(ErrorKind::InvalidInput, "CLI arguments failed").into())
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
        return Err(std::io::Error::new(ErrorKind::AlreadyExists, "Could not create file").into());
    };

    let container = Container::generate(args.get(1).unwrap())?;
    if container.contains("no such container") {
        return Err(
            std::io::Error::new(ErrorKind::NotFound, "Container could not be found").into(),
        );
    };
    let mut file = std::fs::File::create(&path)?;
    file.write_all(container.as_bytes())?;

    Ok(())
}
