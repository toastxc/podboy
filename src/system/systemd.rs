use std::fs::{self};

use super::bash::bash_exec;

pub fn create_systemd(container: &str, script: &str) {
    let user = bash_exec("echo $USER").unwrap();

    let path = format!("/home/{user}/.config/systemd/user/{container}.service");

    if fs::read(&path).is_ok() {
        println!("Daemon script already exits!");
        return;
    };

    fs::File::create(&path).unwrap();

    fs::write(&path, script).unwrap();
}

pub fn rm_systemd(container: &str) {
    let user = bash_exec("echo $USER").unwrap();
    let path = format!("/home/{user}/.config/systemd/user/{container}.service");

    if let Err(e) = fs::read(&path) {
        println!("Could not find file\n{e}");
        return;
    };

    let res = match fs::remove_file(&path) {
        Ok(_) => String::from("Sucessfully deleted script"),
        Err(e) => e.to_string(),
    };

    println!("{res}");
}

pub fn list_systemd() -> String {
    let user = bash_exec("echo $USER").unwrap();
    let path = format!("/home/{user}/.config/systemd/user/");

    let dir = std::fs::read_dir(path).unwrap();

    let mut master = String::new();

    for x in dir.flatten() {
        if let Some(name) = x.file_name().to_str() {
            master += &format!("\n{name}")
        }
    }
    master
}

pub fn systemd_reset() {
    bash_exec("systemctl --user daemon-reload").unwrap();
}
