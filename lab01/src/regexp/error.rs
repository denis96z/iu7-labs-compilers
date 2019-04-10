use std::error::Error;
use std::{error, fmt};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ParseExpError {
    index: usize,
    message: String,
}

impl ParseExpError {
    pub fn new(index: usize) -> Self {
        ParseExpError {
            index,
            message: format!("invalid character at {}", index),
        }
    }
}

impl fmt::Display for ParseExpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for ParseExpError {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
