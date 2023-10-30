use podboy::result::ErrorMsg;
use podboy::{error, HELP};

// true is for session
pub const KW_SYSTEMD: [(&str, bool); 7] = [
    ("start", false),
    ("enable", false),
    ("status", true),
    ("stop", false),
    ("disable", false),
    ("restart", false),
    ("edit", true),
];
pub const KW_PODMAN: [(&str, bool); 3] = [("logs", false), ("exec", true), ("attach", true)];
pub const KW_HA: [&str; 4] = ["regen", "gen", "rm", "ls"];
pub const KW_GENERAL: [&str; 2] = ["version", "help"];

pub fn custom_contains_bool(
    input: Vec<(&str, bool)>,
    value: impl Into<String> + Clone,
) -> Option<(String, bool)> {
    for item in input.into_iter() {
        if item.0 == value.clone().into() {
            return Some((item.0.to_string(), item.1));
        };
    }
    None
}
pub fn custom_contains(input: Vec<&str>, value: impl Into<String> + Clone) -> Option<String> {
    for item in input.into_iter() {
        if item == value.clone().into() {
            return item.to_string().into();
        };
    }
    None
}

fn main() {
    if let Err(error) = main_wrap() {
        println!("{error}");
    }
}

fn main_wrap() -> podboy::result::Result<()> {
    let mut input: Vec<String> = std::env::args().collect();
    // remove useless argument
    input.remove(0);

    if input.is_empty() {
        error!(HELP);
    };

    let argument = input.get(0).unwrap().to_ascii_lowercase();

    match (
        custom_contains_bool(KW_PODMAN.to_vec(), &argument),
        custom_contains_bool(KW_SYSTEMD.to_vec(), &argument),
        custom_contains(KW_HA.to_vec(), &argument),
        custom_contains(KW_GENERAL.to_vec(), &argument),
    ) {
        (Some((_, attach)), _, _, _) => {
            if input.len() < 2 {
                error!(ErrorMsg::CLI_MISUSE);
            }
            podboy::run_dual(attach, "podman", input).map(|_| ())
        }
        (_, Some((_, attach)), _, _) => {
            if input.len() < 2 {
                error!(ErrorMsg::CLI_MISUSE);
            }
            podboy::run_dual(attach, "systemctl --user", input).map(|_| ())
        }
        (_, _, Some(cmd), _) => match cmd.as_str() {
            "regen" | "gen" | "rm" | "ls" => podboy::ha::run(input),
            _ => {
                error!(ErrorMsg::CLI_MISUSE);
            }
        },
        (_, _, _, Some(arg)) => {
            match arg.as_str() {
                "version" => println!("{}", env!("CARGO_PKG_VERSION")),
                "help" => println!("{HELP}"),
                _ => unreachable!(),
            };
            Ok(())
        }
        _ => {
            error!(ErrorMsg::CLI_UNKNOWN);
        }
    }
}
