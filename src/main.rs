// stdin
use std::env::Args;

use std::io::stdin;

pub mod bash;
pub mod systemd;

use systemd::rm_systemd;

use crate::{bash::bash_exec, systemd::create_systemd};

const HELP: &str = "generate <container>
remove <container>

start <container>
stop <container>

enable <container>
disabe <container>";

fn main() {
    let args: Vec<String> = argon(std::env::args());

    if args.is_empty() {
        println!("{HELP}");
        return;
    };

    if args.len() < 2 {
        println!("{HELP}");
        return;
    };

    match &args[0] as &str {
        "generate" | "gen" | "g" => cli_generate(args),
        "remove" | "rm" | "r" => rm_systemd(&args[1]),
        _ => cli_systemd(args),
    }
}

// alias for basic systemd stuff
fn cli_systemd(args: Vec<String>) {
    let wordlist = vec!["start", "enable", "stop", "disable"];

    let arg = format!("systemctl --user {} {}", args[0], args[1]);
    println!("{arg}");
    if wordlist.contains(&args[0].as_str()) {
        bash_exec(&arg).unwrap();
    };
}

fn cli_generate(args: Vec<String>) {
    let script = match { bash_exec(&format!("podman generate systemd {}", &args[1])) } {
        Ok(a) => a,
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    println!(
        "    Successfully generated
    the container {} will be pushed to user systemd
    Do you wish to continue?",
        args[1]
    );

    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();

    if let "y" | "Y" | "yes" | "Yes" = user_input.trim() as &str {
        println!("generating pb_{}.service...", args[1]);
        create_systemd(&args[1], &script);
    };
}

fn argon(args: Args) -> Vec<String> {
    let mut vec = Vec::new();

    for x in args {
        vec.push(x);
    }
    vec.remove(0);
    vec
}
