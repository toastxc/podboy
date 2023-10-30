use std::fmt::Formatter;
use std::string::FromUtf8Error;
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Utf8(FromUtf8Error),
    Custom(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let a = match self {
            Error::Io(_) | Error::Utf8(_) => format!("{:?}", self),
            Error::Custom(data) => format!("{data}\nTry running 'help'"),
        };
        write!(f, "{}", a)
    }
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
impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Custom(value)
    }
}
impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Custom(value.to_string())
    }
}

#[macro_export]
macro_rules! error {
    ($error:expr) => {
        return Err($error.into())
    };
}
#[derive(Debug)]
pub struct ErrorMsg;

impl ErrorMsg {
    pub const FILE_NOT_FOUND: &str = "The file given does not exist";
    pub const FILE_INVALID: &str = "The file exists, but cannot be processed";
    pub const FILE_EXISTS: &str =
        "Could not create file, a file already exists with the same name and path";
    pub const CLI_MISUSE: &str = "The command line arguments given are malformed or incorrect";
    pub const CLI_UNKNOWN: &str = "The command line argument is not one of the available options";
    pub const INVALID_CONTAINER: &str = "The given container does not exist";
}
