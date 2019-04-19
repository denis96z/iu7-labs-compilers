use std::cmp::Ordering;
use std::error::Error;
use std::str::FromStr;

use super::errs;
use super::types;

#[derive(Clone, Debug)]
pub struct Value {
    symbol: types::Symbol,
}

impl Value {
    pub const EMPTY: types::SymbolRef = "ε";
    pub const SPECIAL: types::SymbolRef = "#";

    fn new(symbol: types::Symbol) -> Self {
        Value { symbol }
    }

    pub fn symbol(&self) -> &types::Symbol {
        &self.symbol
    }
}

impl FromStr for Value {
    type Err = errs::ParseExpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(errs::ParseExpError::new(0))
        } else if s.len() > 1 {
            Err(errs::ParseExpError::new(1))
        } else {
            if is_value(s) {
                Ok(Value {
                    symbol: s.to_string(),
                })
            } else {
                Err(errs::ParseExpError::new(0))
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
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.symbol.partial_cmp(&other.symbol)
    }
}

pub fn is_value(s: &str) -> bool {
    if s == Value::EMPTY || s == Value::SPECIAL {
        return true;
    } else {
        for c in s.chars() {
            if !c.is_alphabetic() {
                return false;
            }
        }
        return true;
    }
}

mod tests {
    use super::*;

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
}
