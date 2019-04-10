use crate::regexp::error::ParseRegExpError;
use std::str::FromStr;

pub mod error;

#[derive(PartialEq, Debug)]
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
                    _ => {}
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
                    _ => {
                        nodes.push(BinTree::from_element(cur));
                    }
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
                    _ => return Err(Self::make_parse_err(&s, i)),
                }
            } else if is_bin_op(cur) {
                match prev {
                    Some(c) => {
                        operations.push(cur);
                    }
                    _ => return Err(Self::make_parse_err(&s, i)),
                }
            } else if cur == ')' {
                match operations.pop() {
                    Some(op) => {
                        if op != '(' {
                            return Err(Self::make_parse_err(&s, i));
                        }
                    }
                    _ => return Err(Self::make_parse_err(&s, i)),
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

#[derive(PartialEq, Debug)]
enum BinTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

impl<T> BinTree<T> {
    fn from(element: T, left: BinTree<T>, right: BinTree<T>) -> Self {
        BinTree::NonEmpty(Box::new(TreeNode {
            element,
            left,
            right,
        }))
    }

    fn from_element(element: T) -> Self {
        BinTree::NonEmpty(Box::new(TreeNode::new(element)))
    }

    fn from_elements(element: T, left: T, right: T) -> Self {
        BinTree::from(
            element,
            BinTree::from_element(left),
            BinTree::from_element(right),
        )
    }

    fn from_element_with_left(element: T, left: T) -> Self {
        BinTree::from(element, BinTree::from_element(left), BinTree::Empty)
    }

    fn from_element_with_right(element: T, right: T) -> Self {
        BinTree::from(element, BinTree::Empty, BinTree::from_element(right))
    }
}

#[derive(PartialEq, Debug)]
struct TreeNode<T> {
    element: T,
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> TreeNode<T> {
    fn new(element: T) -> Self {
        TreeNode {
            element,
            left: BinTree::Empty,
            right: BinTree::Empty,
        }
    }
}

fn is_un_op(c: char) -> bool {
    c == '*'
}

fn is_bin_op(c: char) -> bool {
    c == '|' || c == '.'
}

#[test]
fn test_regexp_from_str_simple() {
    let cases = vec![
        ("a", BinTree::from_element('a')),
        ("(a)", BinTree::from_element('a')),
        ("ab", BinTree::from_elements('.', 'a', 'b')),
        ("a|b", BinTree::from_elements('|', 'a', 'b')),
        ("a*", BinTree::from_element_with_left('*', 'a')),
    ];

    for case in cases {
        info!("Test case {:#?}", case);

        match RegExp::from_str(case.0) {
            Ok(r) => {
                assert_eq!(r.tree, case.1);
            }
            _ => unreachable!(),
        };
    }
}

#[test]
fn test_regexp_from_str_complex() {
    let cases = vec![
        (
            "a|b|c",
            BinTree::from(
                '|',
                BinTree::from_elements('|', 'a', 'b'),
                BinTree::from_element('c'),
            ),
        ),
        (
            "abc",
            BinTree::from(
                '.',
                BinTree::from_elements('.', 'a', 'b'),
                BinTree::from_element('c'),
            ),
        ),
        (
            "(a|b)(c|d)",
            BinTree::from(
                '.',
                BinTree::from_elements('|', 'a', 'b'),
                BinTree::from_elements('|', 'c', 'd'),
            ),
        ),
        (
            "(ab)(cd)",
            BinTree::from(
                '.',
                BinTree::from_elements('.', 'a', 'b'),
                BinTree::from_elements('.', 'c', 'd'),
            ),
        ),
        (
            "(ab)*",
            BinTree::from('*', BinTree::from_elements('.', 'a', 'b'), BinTree::Empty),
        ),
        (
            "((ab)*a)|(ab)",
            BinTree::from(
                '|',
                BinTree::from(
                    '.',
                    BinTree::from('*', BinTree::from_elements('.', 'a', 'b'), BinTree::Empty),
                    BinTree::from_element('a'),
                ),
                BinTree::from_elements('.', 'a', 'b'),
            ),
        ),
    ];

    for case in cases {
        info!("Test case {:#?}", case);

        match RegExp::from_str(case.0) {
            Ok(r) => {
                assert_eq!(r.tree, case.1);
            }
            _ => unreachable!(),
        };
    }
}

#[test]
fn test_regexp_from_str_error() {
    let cases = vec![
        ("a**", RegExp::make_parse_err("a**", 2)),
        (")", RegExp::make_parse_err(")", 0)),
    ];

    for case in cases {
        info!("Test case {:#?}", case);

        match RegExp::from_str(case.0) {
            Err(e) => {
                assert_eq!(e, case.1);
            }
            _ => unreachable!(),
        }
    }
}
