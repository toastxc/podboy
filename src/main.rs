mod bash;
mod processing;

use crate::bash::{Bash, Container};
use clap::{Arg, Command};

pub type Result<T> = core::result::Result<T, anyhow::Error>;

fn doc(i: impl Into<String>) -> String {
    format!("{} a container service (systemd)", i.into())
}

fn main() {
    let cmd = Command::new("podboy")
        // general
        .version(clap::crate_version!())
        // systemd section
        .subcommand(
            Command::new("start")
                .arg(Arg::new("cname"))
                .about(doc("start")),
        )
        .subcommand(
            Command::new("stop")
                .arg(Arg::new("cname"))
                .about(doc("stop")),
        )
        .subcommand(
            Command::new("enable")
                .arg(Arg::new("cname"))
                .about(doc("enable")),
        )
        .subcommand(
            Command::new("restart")
                .arg(Arg::new("cname"))
                .about(doc("disable")),
        )
        .subcommand(
            Command::new("status")
                .arg(Arg::new("cname"))
                .about("check status for a container service (systemd)"),
        )
        // service files
        .subcommand(
            Command::new("gen")
                .arg(Arg::new("cname"))
                .about("generate and install a .service file for a container"),
        )
        .subcommand(
            Command::new("regen")
                .arg(Arg::new("cname"))
                .about("force generate and install a .service file for a container"),
        )
        .subcommand(
            Command::new("rm")
                .arg(Arg::new("cname"))
                .about("removes the .service file for a container"),
        )
        .subcommand(Command::new("ls").about("list containers (with HA)"));

    let matcher = cmd.clone().get_matches();

    let Some((matcher, args)) = matcher.subcommand() else {
        return;
    };

    if matcher == "ls" {
        let containers: String = std::fs::read_dir(Container::dir().unwrap())
            .unwrap()
            .filter_map(|a| a.ok()?.file_name().into_string().ok())
            .filter(|a| a.contains(".container"))
            .map(|a| {
                a.split('.')
                    .collect::<Vec<&str>>()
                    .first()
                    .unwrap()
                    .to_string()
            })
            .map(|a| "\n".to_string() + &a)
            .collect();
        println!("CONTAINERS:{containers}");
        return;
    }

    let cname = match args.get_one::<String>("cname") {
        None => {
            println!("argument missing! you need to specify container name");
            return;
        }
        Some(a) => a,
    };

    if let Err(err) = run(matcher, cname) {
        println!("{err}");
    }
}

fn run(matcher: &str, cname: &String) -> Result<()> {
    Bash::exec("systemctl --user daemon-reload").map(|_| ())?;

    match matcher {
        "start" | "stop" | "enable" | "disable" | "restart" => {
            Bash::exec(format!("systemctl --user {matcher} {cname}.container")).map(|_| ())
        }
        "status" => Bash::spawn(format!("systemctl --user status {cname}.container")).map(|_| ()),
        "gen" => processing::gen(cname),
        "regen" => {
            processing::rm(cname)?;
            processing::gen(cname)
        }
        "rm" => processing::rm(cname),
        _ => Ok(()),
    }
}
