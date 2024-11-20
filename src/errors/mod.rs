use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum Errors {
    InvalidArgument(String)
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg)
        }
    }
}

impl Error for Errors {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            _ => None
        }
    }
}