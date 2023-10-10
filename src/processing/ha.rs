use crate::bash::Bash;
use crate::result::Result;
use crate::HELP;
pub fn run(args: Vec<String>) -> Result<()> {
    match args.get(1).unwrap().as_str() {
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
            println!("{}", Bash::cmd(&format!("ls {}", container_dir()?), false)?.unwrap())
        }
        _ => {
            println!("{HELP}");
        }
    }
    Ok(())
}

fn rm(args: Vec<String>) -> Result<()> {
    let path = container_path(args.get(2).unwrap())?;
    std::fs::read(&path)?;
    std::fs::remove_file(&path)?;
    Ok(())
}
fn container_path(container: &str) -> Result<String> {
    Ok(format!("{}/{container}", container_dir()?))

}
fn container_dir() -> Result<String> {
    Ok(format!(
        "/home/{}/.config/systemd/user",
        Bash::username()?,
    ))
}


pub fn gen(args: Vec<String>) -> Result<()> {
    let path = format!(
        "/home/{}/.config/systemd/user/{}.service",
        Bash::username()?,
        args.get(2).unwrap()
    );
    if std::fs::read(&path).is_ok() {
        println!("Daemon script already exits!");
    } else {
        std::fs::File::create(&path)?;
        std::fs::write(&path, Bash::cmd(
            &format!("podman generate systemd {} --restart-policy always", args.get(2).unwrap()),
            false,
        )?
        .unwrap())?;
    };
    Ok(())
}
