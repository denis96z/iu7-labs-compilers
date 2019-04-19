use std::cmp::Ordering;
use std::error::Error;
use std::str::FromStr;

use super::errs;
use super::types;

#[derive(Clone, Debug)]
pub struct Operator {
    symbol: types::Symbol,
    priority: Priority,
    associativity: Associativity,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct Priority(u8);

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Associativity {
    Left,
    Right,
}

impl Operator {
    pub const OPENING_PARENTHESIS: types::SymbolRef = "(";
    pub const CLOSING_PARENTHESIS: types::SymbolRef = ")";

    pub fn symbol(&self) -> &types::Symbol {
        &self.symbol
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

    pub fn is_unary(&self) -> bool {
        self.symbol == "*"
    }

    pub fn is_binary(&self) -> bool {
        self.symbol == "|" || self.symbol == "."
    }
}

impl PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if *self == *other {
            Some(Ordering::Equal)
        } else {
            if self.priority < other.priority {
                Some(Ordering::Less)
            } else if self.priority > other.priority {
                Some(Ordering::Greater)
            } else {
                None
            }
        }
    }
}

impl FromStr for Operator {
    type Err = errs::ParseExpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Operator {
                symbol: s.to_string(),
                priority: Priority(1),
                associativity: Associativity::Left,
            }),

            "." => Ok(Operator {
                symbol: s.to_string(),
                priority: Priority(2),
                associativity: Associativity::Left,
            }),

            "*" => Ok(Operator {
                symbol: s.to_string(),
                priority: Priority(3),
                associativity: Associativity::Right,
            }),

            "(" => Ok(Operator {
                symbol: s.to_string(),
                priority: Priority(0),
                associativity: Associativity::Left,
            }),

            ")" => Ok(Operator {
                symbol: s.to_string(),
                priority: Priority(0),
                associativity: Associativity::Left,
            }),

            _ => Err(errs::ParseExpError::new(0)),
        }
    }
}

pub fn is_operator(s: &str) -> bool {
    vec!["|", ".", "*", "(", ")"].contains(&s)
}

pub fn is_unary_operator(s: &str) -> bool {
    vec!["*"].contains(&s)
}

pub fn is_binary_operator(s: &str) -> bool {
    vec!["|", "."].contains(&s)
}

pub fn is_opening_parenthesis(s: &str) -> bool {
    s == "("
}

pub fn is_closing_parenthesis(s: &str) -> bool {
    s == ")"
}

pub fn is_left_associative(operator: &Operator) -> bool {
    operator.associativity == Associativity::Left
}

pub fn is_right_associative(operator: &Operator) -> bool {
    operator.associativity == Associativity::Right
}

mod tests {
    use super::*;

    #[test]
    fn operator_from_str() {
        assert_eq!(Operator::from_str("|").is_ok(), true);
        assert_eq!(Operator::from_str(".").is_ok(), true);
        assert_eq!(Operator::from_str("*").is_ok(), true);
        assert_eq!(Operator::from_str("(").is_ok(), true);
        assert_eq!(Operator::from_str(")").is_ok(), true);
        assert_eq!(Operator::from_str("").is_err(), true);
    }

    #[test]
    fn operator_cmp() {
        let un = Operator::from_str("|").unwrap();
        let mu = Operator::from_str(".").unwrap();
        let it = Operator::from_str("*").unwrap();

        assert_eq!(un < mu, true);
        assert_eq!(it > mu, true);
        assert_eq!(Operator::from_str("|").unwrap() == un, true);
    }
}
