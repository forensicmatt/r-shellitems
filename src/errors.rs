use std::fmt;
use std::fmt::Display;
use std::io;

#[derive(Debug)]
pub enum ErrorKind {
    IoError,
}

// Lnk Parsing Error
#[derive(Debug)]
pub struct ShellItemError {
    // Formated error message
    pub message: String,
    // The type of error
    pub kind: ErrorKind,
    // Any additional information passed along, such as the argument name that caused the error
    pub info: Option<Vec<String>>,
}

impl From<io::Error> for ShellItemError {
    fn from(err: io::Error) -> Self {
        ShellItemError {
            message: format!("{}",err),
            kind: ErrorKind::IoError,
            info: None,
        }
    }
}

impl Display for ShellItemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { writeln!(f, "{}", self.message) }
}
