use super::types;
use std::error::Error;
use std::str::FromStr;
use std::{cmp, error, fmt};

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub struct Priority(u8);

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Associativity {
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub struct Operator {
    symbol: types::Symbol,
    priority: Priority,
    associativity: Associativity,
}

impl Operator {
    pub fn symbol(&self) -> types::Symbol {
        self.symbol
    }

    pub fn associativity(&self) -> Associativity {
        self.associativity
    }
}

impl PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if *self == *other {
            Some(cmp::Ordering::Equal)
        } else {
            if self.priority < other.priority {
                Some(cmp::Ordering::Less)
            } else if self.priority > other.priority {
                Some(cmp::Ordering::Greater)
            } else {
                None
            }
        }
    }
}

impl FromStr for Operator {
    type Err = ParseOperatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Operator {
                symbol: s.chars().next().unwrap(),
                priority: Priority(1),
                associativity: Associativity::Left,
            }),

            "." => Ok(Operator {
                symbol: s.chars().next().unwrap(),
                priority: Priority(2),
                associativity: Associativity::Left,
            }),

            "*" => Ok(Operator {
                symbol: s.chars().next().unwrap(),
                priority: Priority(3),
                associativity: Associativity::Right,
            }),

            _ => Err(ParseOperatorError::new(0)),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ParseOperatorError {
    index: usize,
}

impl ParseOperatorError {
    fn new(index: usize) -> Self {
        ParseOperatorError { index }
    }
}

impl fmt::Display for ParseOperatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}. index: {}", self.description(), self.index)
    }
}

impl error::Error for ParseOperatorError {
    fn description(&self) -> &str {
        "invalid character"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[test]
fn test_operator_from_str() {
    assert_eq!(Operator::from_str("|").is_ok(), true);
    assert_eq!(Operator::from_str(".").is_ok(), true);
    assert_eq!(Operator::from_str("*").is_ok(), true);
    assert_eq!(Operator::from_str("").is_err(), true);
}

#[test]
fn test_operator_cmp() {
    let un = Operator::from_str("|").unwrap();
    let mu = Operator::from_str(".").unwrap();
    let it = Operator::from_str("*").unwrap();

    assert_eq!(un < mu, true);
    assert_eq!(it > mu, true);
    assert_eq!(Operator::from_str("|").unwrap() == un, true);
}
