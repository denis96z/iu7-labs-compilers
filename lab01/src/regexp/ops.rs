use std::error::Error;
use std::str::FromStr;
use std::{cmp, error, fmt};

use super::error::ParseExpError;
use super::types;

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
    pub const OPENING_PARENTHESIS: types::Symbol = '(';
    pub const CLOSING_PARENTHESIS: types::Symbol = ')';

    pub fn symbol(&self) -> types::Symbol {
        self.symbol
    }

    pub fn associativity(&self) -> Associativity {
        self.associativity
    }

    pub fn is_opening_parenthesis(&self) -> bool {
        self.symbol == Self::OPENING_PARENTHESIS
    }

    pub fn is_closing_parenthesis(&self) -> bool {
        self.symbol == Self::CLOSING_PARENTHESIS
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
    type Err = ParseExpError;

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

            "(" => Ok(Operator {
                symbol: s.chars().next().unwrap(),
                priority: Priority(0),
                associativity: Associativity::Left,
            }),

            ")" => Ok(Operator {
                symbol: s.chars().next().unwrap(),
                priority: Priority(0),
                associativity: Associativity::Left,
            }),

            _ => Err(ParseExpError::new(0)),
        }
    }
}

#[inline(always)]
pub fn is_operator(s: &str) -> bool {
    let operators = vec!["|", ".", "*", "(", ")"];
    operators.contains(&s)
}

#[test]
fn test_operator_from_str() {
    assert_eq!(Operator::from_str("|").is_ok(), true);
    assert_eq!(Operator::from_str(".").is_ok(), true);
    assert_eq!(Operator::from_str("*").is_ok(), true);
    assert_eq!(Operator::from_str("(").is_ok(), true);
    assert_eq!(Operator::from_str(")").is_ok(), true);
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
