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

    let mut h = |s: &str| {
        if ops::is_operator(s) {
            let operator = ops::Operator::from_str(s).unwrap();

            if operator.is_opening_parenthesis() {
                ops_stack.push(operator);
            } else if operator.is_closing_parenthesis() {
                while !ops_stack.is_empty() {
                    let top = ops_stack.pop().unwrap();
                    if top.symbol() == ops::Operator::OPENING_PARENTHESIS {
                        break;
                    }
                    exp_queue.push_back(Symbol::Operator(top));
                }
            } else {
                let mut flag = true;

                while !ops_stack.is_empty() && flag {
                    let top = ops_stack.last().unwrap();
                    if top.is_opening_parenthesis() {
                        break;
                    }

                    let left_cond = ops::is_left_associative(&operator) && operator <= *top;
                    let right_cond = ops::is_right_associative(&operator) && operator < *top;

                    if left_cond || right_cond {
                        exp_queue.push_back(Symbol::Operator(ops_stack.pop().unwrap()));
                    } else {
                        flag = false;
                    }
                }

                ops_stack.push(operator);
            };
        } else if vals::is_value(s) {
            let value = vals::Value::from_str(s).unwrap();
            exp_queue.push_back(Symbol::Value(value));
        };
    };

    let mut prev = '\0';
    for (index, cur) in s.char_indices() {
        let cur_str = &cur.to_string();

        if vals::is_value(cur_str) {
            let prev_str = &prev.to_string();

            if vals::is_value(prev_str)
                || ops::is_unary_operator(prev_str)
                || ops::is_closing_parenthesis(prev_str)
            {
                h(".");
            }

            h(cur_str);
        } else if ops::is_operator(cur_str) {
            h(cur_str);
        } else {
            unimplemented!(); //TODO handle error
        }

        prev = cur;
    }

    while !ops_stack.is_empty() {
        let top = ops_stack.pop().unwrap();

        if top.is_opening_parenthesis() {
            unimplemented!() //TODO handle error
        } else {
            exp_queue.push_back(Symbol::Operator(top));
        }
    }

    exp_queue
}
