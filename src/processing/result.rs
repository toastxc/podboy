use std::string::FromUtf8Error;
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Utf8(FromUtf8Error),
}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Self::Utf8(value)
    }
}

#[macro_export]
macro_rules! error {
    ($error:expr) => {{
        println!("{}\n\nrun help for help", $error);
        return Ok(());
    }};
}
#[macro_export]
macro_rules! error_help {
    () => {{
        println!("{HELP}");
        return Ok(());
    }};
}

pub struct ErrorMsg;

impl ErrorMsg {
    pub const FILE_NOT_FOUND: &str = "The file given does not exist";
    pub const FILE_INVALID: &str = "The file exists, but cannot be processed";
    pub const FILE_EXISTS: &str =
        "Could not create file, a file already exists with the same name and path";
    pub const CLI_MISUSE: &str = "The command line arguments given are malformed or incorrect";
    pub const INVALID_CONTAINER: &str = "The given container does not exist";
}
