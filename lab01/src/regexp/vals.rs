use super::types;
use std::error::Error;
use std::str::FromStr;
use std::{cmp, error, fmt};

struct Value {
    symbol: types::Symbol,
}

impl Value {
    fn new(symbol: types::Symbol) -> Self {
        Value { symbol }
    }

    pub fn symbol(&self) -> types::Symbol {
        self.symbol
    }
}

impl FromStr for Value {
    type Err = ParseValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 1 {
            Err(ParseValueError::new(0))
        } else if s.len() > 1 {
            Err(ParseValueError::new(1))
        } else {
            let c = s.chars().next().unwrap();
            if c.is_alphabetic() {
                Ok(Value::new(c))
            } else {
                Err(ParseValueError::new(0))
            }
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.symbol.partial_cmp(&other.symbol)
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ParseValueError {
    index: usize,
}

impl ParseValueError {
    fn new(index: usize) -> Self {
        ParseValueError { index }
    }
}

impl fmt::Display for ParseValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}. index: {}", self.description(), self.index)
    }
}

impl error::Error for ParseValueError {
    fn description(&self) -> &str {
        "invalid character"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[test]
fn test_value_from_str() {
    assert_eq!(Value::from_str("a").is_ok(), true);
    assert_eq!(Value::from_str("&").is_err(), true);
    assert_eq!(Value::from_str("").is_err(), true);
    assert_eq!(Value::from_str("abc").is_err(), true);
}

fn test_value_cmp() {
    let (s1, s2) = ("a", "b");

    assert_eq!(
        Value::from_str(s1).unwrap() == Value::from_str(s1).unwrap(),
        true
    );
    assert_eq!(
        Value::from_str(s1).unwrap() != Value::from_str(s2).unwrap(),
        true
    );
}
