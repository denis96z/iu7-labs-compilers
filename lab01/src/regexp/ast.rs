use std::collections::VecDeque;
use std::error::Error;
use std::str::FromStr;
use std::{error, fmt};

use crate::tree::{BinTree, TreeNode};

use super::error::ParseExpError;
use super::{ops, vals};

#[derive(Debug)]
enum Symbol {
    Value(vals::Value),
    Operator(ops::Operator),
}

#[derive(PartialEq, Debug)]
pub struct AbstractSyntaxTree {
    root: BinTree<char>,
}

impl AbstractSyntaxTree {
    pub fn new() -> Self {
        AbstractSyntaxTree {
            root: BinTree::new(),
        }
    }
}

impl FromStr for AbstractSyntaxTree {
    type Err = ParseExpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

fn make_rpn(s: &str) -> VecDeque<Symbol> {
    let mut ops_stack: Vec<ops::Operator> = Vec::new();
    let mut exp_queue: VecDeque<Symbol> = VecDeque::with_capacity(s.len());

    let h = |c: char| -> Result<(), ()> {
        if c == ops::Operator::OPENING_PARENTHESIS {
            let r = ops::Operator::from_str(&c.to_string()).unwrap();
            ops_stack.push(r);
        } else if c == ops::Operator::CLOSING_PARENTHESIS {
            while !ops_stack.is_empty() {
                let op = ops_stack.pop().unwrap();
                if op.symbol() == ops::Operator::OPENING_PARENTHESIS {
                    break;
                }
                exp_queue.push_back(Symbol::Operator(op));
            }
        }

        match vals::Value::from_str(&c.to_string()) {
            Ok(value) => {
                exp_queue.push_back(Symbol::Value(value));
                return Ok(());
            }
            _ => {}
        }

        match ops::Operator::from_str(&c.to_string()) {
            Ok(operator) => {
                while !ops_stack.is_empty() {
                    let top_operator = ops_stack.last().unwrap();
                    if top_operator.is_opening_parenthesis() {
                        break;
                    }

                    let left_cond = operator.associativity() == ops::Associativity::Left
                        && operator <= *top_operator;
                    let right_cond = operator.associativity() == ops::Associativity::Right
                        && operator < *top_operator;

                    if left_cond || right_cond {
                        exp_queue.push_back(Symbol::Operator(ops_stack.pop().unwrap()));
                    }
                }

                return Ok(());
            }
            _ => {}
        }

        Err(())
    };

    let mut prev = '\0';
    for (index, cur) in s.char_indices() {
        let cur_str = &cur.to_string();

        if vals::is_value(cur_str) {
            //TODO handle value
        } else if ops::is_operator(cur_str) {
            //TODO handle operator
        } else {
            unimplemented!(); //TODO handle error
        }
    }

    exp_queue
}
