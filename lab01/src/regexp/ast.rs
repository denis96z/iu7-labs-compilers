use std::collections::VecDeque;
use std::error::Error;
use std::str::FromStr;

use crate::tree;

use super::errs;
use super::{ops, vals};

type Stack<T> = Vec<T>;
type Queue<T> = std::collections::VecDeque<T>;

#[derive(PartialEq, Clone, Debug)]
pub struct AbstractSyntaxTree {
    root: tree::BinTree<TreeNode>,
}

impl AbstractSyntaxTree {
    pub fn new() -> Self {
        AbstractSyntaxTree {
            root: tree::BinTree::new(),
        }
    }
}

impl FromStr for AbstractSyntaxTree {
    type Err = errs::ParseExpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AbstractSyntaxTree {
            root: make_tree(make_rpn(s)?)?,
        })
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Symbol {
    Value(vals::Value),
    Operator(ops::Operator),
}

impl Symbol {
    fn from_value_str(s: &str) -> Result<Self, errs::ParseExpError> {
        let value = vals::Value::from_str(s)?;
        Ok(Symbol::Value(value))
    }

    fn from_operator_str(s: &str) -> Result<Self, errs::ParseExpError> {
        let operator = ops::Operator::from_str(s)?;
        Ok(Symbol::Operator(operator))
    }
}

fn make_rpn(s: &str) -> Result<Queue<Symbol>, errs::ParseExpError> {
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
                    if top == ops::OPENING_PARENTHESIS {
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

                    let left_cond =
                        operator.associativity() == ops::Associativity::Left && operator <= *top;
                    let right_cond =
                        operator.associativity() == ops::Associativity::Right && operator < *top;

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
            return Err(errs::ParseExpError::new(index));
        }

        prev = cur;
    }

    while !operators.is_empty() {
        let top = operators.pop().unwrap();

        if top.is_opening_parenthesis() {
            return Err(errs::ParseExpError::new(s.len()));
        } else {
            symbols.push_back(Symbol::Operator(top));
        }
    }

    Ok(symbols)
}

#[derive(PartialEq, Clone, Debug)]
struct TreeNode(usize, Symbol);

fn make_tree(symbols: Queue<Symbol>) -> Result<tree::BinTree<TreeNode>, errs::ParseExpError> {
    let mut stack = Stack::new();

    let mut index = 0usize;

    for symbol in symbols.iter() {
        index += 1;

        match symbol {
            Symbol::Value(value) => {
                stack.push(tree::BinTree::from_element(TreeNode(index, symbol.clone())));
            }

            Symbol::Operator(operator) => {
                if operator.is_unary() {
                    let node = tree::BinTree::from(
                        TreeNode(index, symbol.clone()),
                        stack.pop().unwrap(),
                        tree::BinTree::Empty,
                    );
                    stack.push(node);
                } else if operator.is_binary() {
                    let right_node = stack.pop().unwrap();
                    let left_node = stack.pop().unwrap();

                    let node =
                        tree::BinTree::from(TreeNode(index, symbol.clone()), left_node, right_node);

                    stack.push(node);
                } else {
                    unreachable!();
                }
            }
        }
    }

    Ok(stack.pop().unwrap())
}

#[derive(PartialEq, Eq, Debug)]
struct Params {
    pub is_nullable: bool,
    pub first_pos: Vec<usize>,
    pub last_pos: Vec<usize>,
    pub follow_pos: Vec<usize>,
}

fn make_params_tree(tree: &tree::BinTree<Symbol>) -> tree::BinTree<Params> {
    if tree.is_empty() {
        tree::BinTree::from_element(Params {
            is_nullable: true,
            first_pos: Vec::new(),
            last_pos: Vec::new(),
            follow_pos: Vec::new(),
        })
    } else {
        unimplemented!()
    }
}

mod tests {
    use super::*;

    #[test]
    fn tree_new() {
        AbstractSyntaxTree::new();
    }

    #[test]
    fn tree_from_str() {
        let cases = vec!["((a|b)*abb)#"];

        for case in cases {
            AbstractSyntaxTree::from_str(case).unwrap();
        }
    }

    #[test]
    fn make_rpn() {
        let cases = vec![
            (
                "((a|b)c|d)#",
                vec![
                    Symbol::from_value_str("a").unwrap(),
                    Symbol::from_value_str("b").unwrap(),
                    Symbol::from_operator_str("|").unwrap(),
                    Symbol::from_value_str("c").unwrap(),
                    Symbol::from_operator_str(".").unwrap(),
                    Symbol::from_value_str("d").unwrap(),
                    Symbol::from_operator_str("|").unwrap(),
                    Symbol::from_value_str("#").unwrap(),
                    Symbol::from_operator_str(".").unwrap(),
                ]
                .into_iter()
                .collect::<Queue<_>>(),
            ),
            (
                "(a*b|cd)#",
                vec![
                    Symbol::from_value_str("a").unwrap(),
                    Symbol::from_operator_str("*").unwrap(),
                    Symbol::from_value_str("b").unwrap(),
                    Symbol::from_operator_str(".").unwrap(),
                    Symbol::from_value_str("c").unwrap(),
                    Symbol::from_value_str("d").unwrap(),
                    Symbol::from_operator_str(".").unwrap(),
                    Symbol::from_operator_str("|").unwrap(),
                    Symbol::from_value_str("#").unwrap(),
                    Symbol::from_operator_str(".").unwrap(),
                ]
                .into_iter()
                .collect::<Queue<_>>(),
            ),
        ];

        for case in cases {
            assert_eq!(super::make_rpn(case.0).unwrap(), case.1);
        }
    }

    #[test]
    fn make_tree() {
        let cases = vec![(
            vec![
                Symbol::from_value_str("a").unwrap(),
                Symbol::from_operator_str("*").unwrap(),
                Symbol::from_value_str("b").unwrap(),
                Symbol::from_operator_str(".").unwrap(),
                Symbol::from_value_str("c").unwrap(),
                Symbol::from_value_str("d").unwrap(),
                Symbol::from_operator_str(".").unwrap(),
                Symbol::from_operator_str("|").unwrap(),
                Symbol::from_value_str("#").unwrap(),
                Symbol::from_operator_str(".").unwrap(),
            ]
            .into_iter()
            .collect::<Queue<_>>(),
            tree::BinTree::from(
                TreeNode(10, Symbol::from_operator_str(".").unwrap()),
                tree::BinTree::from(
                    TreeNode(8, Symbol::from_operator_str("|").unwrap()),
                    tree::BinTree::from(
                        TreeNode(4, Symbol::from_operator_str(".").unwrap()),
                        tree::BinTree::from_element_with_left(
                            TreeNode(2, Symbol::from_operator_str("*").unwrap()),
                            TreeNode(1, Symbol::from_value_str("a").unwrap()),
                        ),
                        tree::BinTree::from_element(TreeNode(
                            3,
                            Symbol::from_value_str("b").unwrap(),
                        )),
                    ),
                    tree::BinTree::from(
                        TreeNode(7, Symbol::from_operator_str(".").unwrap()),
                        tree::BinTree::from_element(TreeNode(
                            5,
                            Symbol::from_value_str("c").unwrap(),
                        )),
                        tree::BinTree::from_element(TreeNode(
                            6,
                            Symbol::from_value_str("d").unwrap(),
                        )),
                    ),
                ),
                tree::BinTree::from_element(TreeNode(9, Symbol::from_value_str("#").unwrap())),
            ),
        )];

        for case in cases {
            assert_eq!(super::make_tree(case.0).unwrap(), case.1);
        }
    }
}
