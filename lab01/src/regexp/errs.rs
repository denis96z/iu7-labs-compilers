use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(PartialEq, Eq, Clone, Debug)]
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

impl Display for ParseExpError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message)
    }
}

impl Error for ParseExpError {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
