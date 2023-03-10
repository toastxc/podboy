use crate::bash::bash_exec;
use std::fs;

pub fn create_systemd(container: &str, script: &str) {
    let user = bash_exec("echo $USER").unwrap();
    let path = format!("/home/{}/.config/systemd/user/{container}.service", user);

    if fs::read(&path).is_ok() {
        println!("Daemon script already exits!");
        return;
    };

    fs::File::create(&path).unwrap();

    fs::write(&path, script).unwrap();

    let result = match bash_exec("systemctl --user daemon-reload") {
        Ok(_) => String::from("Script successfully added"),
        Err(e) => e.to_string(),
    };

    println!("{result}");
}

pub fn rm_systemd(container: &str) {
    let user = bash_exec("echo $USER").unwrap();
    let path = format!("/home/{}/.config/systemd/user/{container}.service", user);

    if let Err(e) = fs::read(&path) {
        println!("Could not find file\n{e}");
        return;
    };

    let res = match fs::remove_file(&path) {
        Ok(_) => String::from("Sucessfully deleted script"),
        Err(e) => e.to_string(),
    };

    bash_exec("systemctl --user daemon-reload").unwrap();

    println!("{res}");
}
