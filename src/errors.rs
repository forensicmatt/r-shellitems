use std::string::FromUtf8Error;
use std::fmt;
use std::fmt::Display;
use std::io;

#[derive(Debug)]
pub enum ErrorKind {
    IoError,
    Utf16Error
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
impl ShellItemError {
    #[allow(dead_code)]
    pub fn utf16_decode_error(err: String)->Self{
        ShellItemError {
            message: format!("{}",err),
            kind: ErrorKind::Utf16Error,
            info: Some(vec![]),
        }
    }
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
impl From<FromUtf8Error> for ShellItemError {
    fn from(err: FromUtf8Error) -> Self {
        ShellItemError {
            message: format!("{}",err),
            kind: ErrorKind::Utf16Error,
            info: None,
        }
    }
}
impl Display for ShellItemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { writeln!(f, "{}", self.message) }
}
