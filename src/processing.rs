use crate::bash::Container;
use crate::Result;
use std::io::{ErrorKind, Write};

pub fn rm(cname: impl Into<String>) -> Result<()> {
    let path = Container::path(cname)?;
    std::fs::read(&path)?;
    std::fs::remove_file(&path)?;
    Ok(())
}

pub fn gen(cname: impl Into<String>) -> Result<()> {
    let cname = cname.into();
    let path = Container::path(&cname)?;

    if std::fs::read(&path).is_ok() {
        return Err(std::io::Error::new(ErrorKind::AlreadyExists, format!("Could not create file {path}\
File already exists\
the command `regen` will override existing service files of the same name, or use `rm` to remove the old file")).into());
    };

    let container = Container::generate(&cname)?;
    if container.contains("no such container") {
        return Err(
            std::io::Error::new(ErrorKind::NotFound, "Container could not be found").into(),
        );
    };
    let mut file = std::fs::File::create(&path)?;
    file.write_all(container.as_bytes())?;

    Ok(())
}
