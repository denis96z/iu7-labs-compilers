use crate::regexp::error::ParseRegExpError;
use std::str::FromStr;

pub mod error;

pub struct RegExp {
    tree: BinTree<char>,
}

impl RegExp {
    fn make_parse_err(s: &str, index: usize) -> ParseRegExpError {
        if index == s.len() {
            ParseRegExpError::new(&format!("Invalid expression: {0}!", s))
        } else {
            ParseRegExpError::new(&format!("Invalid expression: {0}, position {1}!", s, index))
        }
    }
}

impl FromStr for RegExp {
    type Err = error::ParseRegExpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = Vec::new();
        let mut operations = Vec::new();

        let mut prev: Option<char> = None;
        for (i, cur) in s.char_indices() {
            if cur == '(' {
                match prev {
                    Some(c) => {
                        if c.is_alphabetic() || is_un_op(c) || c == ')' {
                            operations.push('.');
                        }
                    }
                    _ => unreachable!(),
                }
                operations.push(cur);
            } else if cur.is_alphabetic() {
                match prev {
                    Some(c) => {
                        let node = BinTree::from_element(cur);

                        if c.is_alphabetic() || is_un_op(c) || c == ')' {
                            let left_node = nodes.pop().unwrap();
                            nodes.push(BinTree::from('.', left_node, node));
                        } else if is_bin_op(c) {
                            let op = operations.pop().unwrap();
                            let left_node = nodes.pop().unwrap();
                            nodes.push(BinTree::from(op, left_node, node))
                        } else {
                            nodes.push(node);
                        }
                    }
                    _ => unreachable!(),
                }
            } else if is_un_op(cur) {
                match prev {
                    Some(c) => {
                        if c.is_alphabetic() || c == ')' {
                            let left_node = nodes.pop().unwrap();
                            nodes.push(BinTree::from(cur, left_node, BinTree::Empty))
                        } else {
                            return Err(Self::make_parse_err(&s, i));
                        }
                    }
                    _ => unreachable!(),
                }
            } else if is_bin_op(cur) {
                operations.push(cur);
            } else if cur == ')' {
                let op = operations.pop().unwrap();
                if op != '(' {
                    return Err(Self::make_parse_err(&s, i));
                }
            } else {
                return Err(Self::make_parse_err(&s, i));
            }

            prev = Some(cur);
        }

        while !operations.is_empty() {
            let right_node = nodes.pop().unwrap();
            let left_node = nodes.pop().unwrap();

            match operations.pop() {
                Some(c) => {
                    if is_bin_op(c) {
                        nodes.push(BinTree::from(c, left_node, right_node))
                    } else {
                        return Err(Self::make_parse_err(&s, s.len()));
                    }
                }
                _ => return Err(Self::make_parse_err(&s, s.len())),
            };
        }

        match nodes.pop() {
            Some(node) => {
                if nodes.len() == 0 {
                    Ok(RegExp { tree: node })
                } else {
                    Err(Self::make_parse_err(&s, s.len()))
                }
            }
            _ => Err(Self::make_parse_err(&s, s.len())),
        }
    }
}

enum BinTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element: T,
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> BinTree<T> {
    fn from_element(element: T) -> Self {
        BinTree::NonEmpty(Box::new(TreeNode {
            element,
            left: BinTree::Empty,
            right: BinTree::Empty,
        }))
    }

    fn from(element: T, left: BinTree<T>, right: BinTree<T>) -> Self {
        BinTree::NonEmpty(Box::new(TreeNode {
            element,
            left,
            right,
        }))
    }
}

fn is_un_op(c: char) -> bool {
    c == '*'
}

fn is_bin_op(c: char) -> bool {
    c == '|' || c == '.'
}
