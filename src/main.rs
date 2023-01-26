use std::env::Args;
use std::io::stdin;

pub mod bash;
pub mod systemd;

use crate::{
    bash::bash_exec,
    systemd::{create_systemd, rm_systemd},
};

const HELP: &str = "generate <container>
remove <container>
regen <container>
stauts <container>
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
        "regen" | "regenerate" | "rg" => {
            rm_systemd(&args[1]);
            cli_generate(args);
        }
        _ => cli_systemd(args),
    }
}

// alias for basic systemd stuff
fn cli_systemd(args: Vec<String>) {
    let wordlist = vec!["start", "enable", "status", "stop", "disable"];

    let arg = format!("systemctl --user {} {}", args[0], args[1]);

    let result = if wordlist.contains(&args[0].as_str()) {
        match bash_exec(&arg) {
            Ok(a) => status_readable(a),
            Err(e) => e.to_string(),
        }
    } else {
        String::from(HELP)
    };
    println!("{}", result)
}

fn status_readable(a: String) -> String {
    let mut newstr = String::new();
    let mut iter = 8;

    for x in a.lines() {
        newstr += &format!("\n{}", x);
        iter -= 1;

        if iter == 0 {
            return newstr;
        }
    }
    newstr
}

fn cli_generate(args: Vec<String>) {
    let script = match {
        bash_exec(&format!(
            "podman generate systemd {} --restart-policy always",
            &args[1]
        ))
    } {
        Ok(a) => a,
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    println!(
        "Successfully generated script\nThe container `{}` will be added to daemon\nDo you wish to continue?\n(Y/n)",
        args[1]
    );

    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();

    if let "y" | "Y" | "yes" | "Yes" = user_input.trim() as &str {
        println!("Generating {}.service...", args[1]);
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
