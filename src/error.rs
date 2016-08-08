use std::error::Error;
use std::fmt::{Display, Formatter};
use std;

#[derive(Debug)]
pub enum ShellError {
    Cd(String),
    False,
    IOError(String)
}

impl Display for ShellError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

impl std::convert::From<std::io::Error> for ShellError {
    fn from(err: std::io::Error) -> ShellError {
        ShellError::IOError(err.description().to_string())
    }
}

// impl Error for BuiltinError {
//     fn description(&self) -> &str {
//         ""
//     }

// fn cause(&self) -> Option<&Error> { None }
// }
