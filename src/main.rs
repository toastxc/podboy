use podboy::HELP;

// true is for session
pub const KW_SYSTEMD: [(&str, bool); 6] = [
    ("start", false),
    ("enable", false),
    ("status", true),
    ("stop", false),
    ("disable", false),
    ("restart", false),
];
pub const KW_PODMAN: [(&str, bool); 3] = [("logs", false), ("exec", true), ("attach", true)];
pub const KW_HA: [(&str, bool); 5] = [
    ("version", false),
    ("regen", false),
    ("gen", false),
    ("rm", false),
    ("ls", false),
];

pub fn custom_contains(
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

fn main() {
    let input: Vec<String> = std::env::args().collect();
    if input.len() < 2 {
        println!("{HELP}");
        return;
    };
    let a = match (
        custom_contains(KW_PODMAN.to_vec(), input.get(1).unwrap()),
        custom_contains(KW_SYSTEMD.to_vec(), input.get(1).unwrap()),
        custom_contains(KW_HA.to_vec(), input.get(1).unwrap()),
    ) {
        (Some((_, attach)), _, _) => {
            if input.len() < 3 {
                println!("{HELP}");
                return;
            }
            podboy::run_dual(attach, "podman", input).map(|_| ())},
        (_, Some((_, attach)), _) => {
            if input.len() < 3 {
                println!("{HELP}");
                return;
            }
            podboy::run_dual(attach, "systemctl --user", input).map(|_| ())
        }
        (_, _, Some((cmd, _))) => match cmd.as_str() {
            "version" => {
                println!("{}", env!("CARGO_PKG_VERSION"));
                Ok(())
            }
            "regen" | "gen" | "rm" | "ls" => podboy::ha::run(input),
            _ => {
                println!("{HELP}");
                Ok(())
            }
        },
        _ => {
            println!("{HELP}");
            Ok(())
        }
    };
    if let Some(error) = a.err() {
        println!("{:?}", error);
    }
}
