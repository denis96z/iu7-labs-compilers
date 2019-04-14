use std::error::Error;
use std::str::FromStr;
use std::{cmp, error, fmt};

use super::error::ParseExpError;
use super::types;

#[derive(Copy, Clone, Debug)]
pub struct Value {
    symbol: types::Symbol,
}

impl Value {
    pub const EMPTY: types::Symbol = 'ε';
    pub const SPECIAL: types::Symbol = '#';

    fn new(symbol: types::Symbol) -> Self {
        Value { symbol }
    }

    pub fn symbol(&self) -> types::Symbol {
        self.symbol
    }
}

impl FromStr for Value {
    type Err = ParseExpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ParseExpError::new(0))
        } else if s.len() > 1 {
            Err(ParseExpError::new(1))
        } else {
            let c = s.chars().next().unwrap();
            if is_value(&s.to_string()) {
                Ok(Value::new(c))
            } else {
                Err(ParseExpError::new(0))
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

pub fn is_value(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_alphabetic() && c != Value::EMPTY && c != Value::SPECIAL {
            return false;
        }
    }
    true
}

#[test]
fn test_value_from_str() {
    assert_eq!(Value::from_str("a").is_ok(), true);
    assert_eq!(Value::from_str("&").is_err(), true);
    assert_eq!(Value::from_str("").is_err(), true);
    assert_eq!(Value::from_str("abc").is_err(), true);
}

#[test]
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
