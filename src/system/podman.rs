use std::io::stdin;

use crate::system::{bash::bash_exec, systemd::create_systemd};

pub fn podman_generate(args: Vec<String>) {
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
        "Successfully generated script\nThe container `{}` will be added to daemon
                Do you wish to continue?\n(Y/n)",
        args[1]
    );

    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();

    if let "y" | "Y" | "yes" | "Yes" = user_input.trim() as &str {
        println!("Generating {}.service...", args[1]);
        create_systemd(&args[1], &script);
    };
}
