use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ParseRegExpError {
    message: String,
}

impl ParseRegExpError {
    pub fn new(message: &str) -> Self {
        ParseRegExpError {
            message: message.clone().to_string(),
        }
    }
}

impl fmt::Display for ParseRegExpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", &self.message)
    }
}

impl error::Error for ParseRegExpError {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
