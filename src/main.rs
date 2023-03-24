use podboy::system::{
    bash::bash_spawn,
    podman::podman_generate,
    systemd::{list_systemd, rm_systemd, systemd_reset},
};
use std::env::Args;

const HELP: &str = "generate <container>
remove <container>
regen <container>
status <container>
start <container>
stop <container>
enable <container>
disable <container>
list";

const DARRAY: [&str; 6] = ["start", "enable", "status", "stop", "disable", "restart"];

#[tokio::main]
async fn main() {
    let args: Vec<String> = argon(std::env::args());

    if args.is_empty() {
        println!("{HELP}");
        return;
    };

    match &args[0] as &str {
        // no arg commands
        "list" => println!("{}", list_systemd()),

        // 2 arg
        a if args.len() < 2 => {
            println!("{a} requires more arguments")
        }

        a if DARRAY.contains(&a) => cli_systemd(args).await,

        "generate" | "gen" | "g" => podman_generate(args),
        "remove" | "rm" | "r" => rm_systemd(&args[1]),
        "regen" | "regenerate" | "rg" => {
            rm_systemd(&args[1]);
            podman_generate(args);
        }

        // exceptions
        _ => println!("{HELP}"),
    };

    systemd_reset()
}

// alias for basic systemd stuff
async fn cli_systemd(args: Vec<String>) {
    let mut newargs = String::new();
    for x in args {
        newargs += &format!(" {x}");
    }

    let arg = format!("systemctl --user {}", newargs);

    bash_spawn(&arg).await;
}

fn argon(args: Args) -> Vec<String> {
    let mut vec = Vec::new();

    for x in args {
        vec.push(x);
    }
    vec.remove(0);
    vec
}
