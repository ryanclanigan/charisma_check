use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    WriteError(String),
    ReadError(String),
    InvalidFile(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Error::WriteError(e) => write!(f, "Could not write to file. Underlying error: {}", e),
            Error::ReadError(e) => write!(f, "Could not read from file. Underlying error: {}", e),
            Error::InvalidFile(e) => write!(f, "Invalid file name. Underlying error: {}", e),
        }
    }
}

impl std::error::Error for Error {}
