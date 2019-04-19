use std::collections::VecDeque;
use std::error::Error;
use std::str::FromStr;
use std::{error, fmt};

use crate::tree::{BinTree, TreeNode};

use super::error::ParseExpError;
use super::{ops, vals};

type Stack<T> = Vec<T>;
type Queue<T> = std::collections::VecDeque<T>;

#[derive(Clone, Copy, Debug)]
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

fn make_rpn(s: &str) -> Result<Queue<Symbol>, ParseExpError> {
    let mut operators = Stack::new();
    let mut symbols = Queue::with_capacity(s.len());

    let mut h = |s: &str| {
        if ops::is_operator(s) {
            let operator = ops::Operator::from_str(s).unwrap();

            if operator.is_opening_parenthesis() {
                operators.push(operator);
            } else if operator.is_closing_parenthesis() {
                while !operators.is_empty() {
                    let top = operators.pop().unwrap();
                    if top.symbol() == ops::Operator::OPENING_PARENTHESIS {
                        break;
                    }
                    symbols.push_back(Symbol::Operator(top));
                }
            } else {
                let mut flag = true;

                while !operators.is_empty() && flag {
                    let top = operators.last().unwrap();
                    if top.is_opening_parenthesis() {
                        break;
                    }

                    let left_cond = ops::is_left_associative(&operator) && operator <= *top;
                    let right_cond = ops::is_right_associative(&operator) && operator < *top;

                    if left_cond || right_cond {
                        symbols.push_back(Symbol::Operator(operators.pop().unwrap()));
                    } else {
                        flag = false;
                    }
                }

                operators.push(operator);
            };
        } else if vals::is_value(s) {
            let value = vals::Value::from_str(s).unwrap();
            symbols.push_back(Symbol::Value(value));
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
            return Err(ParseExpError::new(index));
        }

        prev = cur;
    }

    while !operators.is_empty() {
        let top = operators.pop().unwrap();

        if top.is_opening_parenthesis() {
            return Err(ParseExpError::new(s.len()));
        } else {
            symbols.push_back(Symbol::Operator(top));
        }
    }

    Ok(symbols)
}

fn make_tree(symbols: &Queue<Symbol>) -> Result<BinTree<Symbol>, ParseExpError> {
    let mut stack = Stack::new();

    for symbol in symbols.iter() {
        match symbol {
            Symbol::Value(value) => {
                stack.push(BinTree::from_element(*symbol));
            }

            Symbol::Operator(operator) => {
                if operator.is_unary() {
                    let node = BinTree::from(*symbol, stack.pop().unwrap(), BinTree::Empty);
                } else if operator.is_binary() {
                    let right_node = stack.pop().unwrap();
                    let left_node = stack.pop().unwrap();

                    let node = BinTree::from(*symbol, left_node, right_node);

                    stack.push(node);
                } else {
                    unreachable!();
                }
            }
        }
    }

    Err(ParseExpError::new(symbols.len()))
}
