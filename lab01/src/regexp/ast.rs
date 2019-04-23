use std::str::FromStr;

use crate::{trees, types, utils};

use super::{errs, ops, vals};

#[derive(PartialEq, Clone, Debug)]
pub struct AbstractSyntaxTree {
    root: trees::BinTree<TreeNode>,
}

impl AbstractSyntaxTree {
    pub fn new() -> Self {
        AbstractSyntaxTree {
            root: trees::BinTree::new(),
        }
    }

    pub fn root(&self) -> &trees::BinTree<TreeNode> {
        &self.root
    }

    pub fn make_params_tree(&self) -> trees::BinTree<Params> {
        let mut t = make_params_tree(&self.root);
        let empty_set = utils::make_empty_set();
        add_follow_pos(&mut t, &self.root, &empty_set, &empty_set);
        return t;
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
pub enum Symbol {
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

fn make_rpn(s: &str) -> Result<types::Queue<Symbol>, errs::ParseExpError> {
    let mut operators = types::Stack::new();
    let mut symbols = types::Queue::with_capacity(s.len());

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
                h(ops::CONCATENATION.symbol());
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
pub struct TreeNode(pub usize, pub Symbol);

fn make_tree(
    symbols: types::Queue<Symbol>,
) -> Result<trees::BinTree<TreeNode>, errs::ParseExpError> {
    let mut stack = types::Stack::new();

    let mut index = 0;

    for symbol in symbols.iter() {
        match symbol {
            Symbol::Value(_) => {
                index += 1;

                stack.push(trees::BinTree::from_element(TreeNode(
                    index,
                    symbol.clone(),
                )));
            }

            Symbol::Operator(operator) => {
                if operator.is_unary() {
                    let node = trees::BinTree::from(
                        TreeNode(0, symbol.clone()),
                        stack.pop().unwrap(),
                        trees::BinTree::Empty,
                    );
                    stack.push(node);
                } else if operator.is_binary() {
                    let right_node = stack.pop().unwrap();
                    let left_node = stack.pop().unwrap();

                    let node =
                        trees::BinTree::from(TreeNode(0, symbol.clone()), left_node, right_node);

                    stack.push(node);
                } else {
                    unreachable!();
                }
            }
        }
    }

    Ok(stack.pop().unwrap())
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Params {
    pub is_nullable: bool,
    pub first_pos: types::Set<usize>,
    pub last_pos: types::Set<usize>,
    pub follow_pos: types::Set<usize>,
}

fn make_params_tree(syntax_tree: &trees::BinTree<TreeNode>) -> trees::BinTree<Params> {
    if syntax_tree.is_empty() {
        return trees::BinTree::from_element(Params {
            is_nullable: true,
            first_pos: utils::make_empty_set(),
            last_pos: utils::make_empty_set(),
            follow_pos: utils::make_empty_set(),
        });
    }

    let current_node = match syntax_tree {
        trees::BinTree::NonEmpty(tree_node) => tree_node,
        _ => unreachable!(),
    };

    match &current_node.element.1 {
        Symbol::Value(_) => trees::BinTree::from_element(Params {
            is_nullable: false,
            first_pos: utils::make_set_from_vec(vec![current_node.element.0]),
            last_pos: utils::make_set_from_vec(vec![current_node.element.0]),
            follow_pos: utils::make_empty_set(),
        }),

        Symbol::Operator(operator) => {
            if *operator == ops::CLOSURE {
                match make_params_tree(&current_node.left_tree) {
                    trees::BinTree::NonEmpty(left_node) => {
                        let params = Params {
                            is_nullable: true,
                            first_pos: left_node.element.first_pos.clone(),
                            last_pos: left_node.element.last_pos.clone(),
                            follow_pos: utils::make_empty_set(),
                        };
                        trees::BinTree::from(
                            params,
                            trees::BinTree::NonEmpty(left_node),
                            trees::BinTree::Empty,
                        )
                    }
                    _ => unreachable!(),
                }
            } else if *operator == ops::CONCATENATION || *operator == ops::ALTERATION {
                match make_params_tree(&current_node.left_tree) {
                    trees::BinTree::NonEmpty(left_node) => {
                        match make_params_tree(&current_node.right_tree) {
                            trees::BinTree::NonEmpty(right_node) => {
                                let params = if *operator == ops::CONCATENATION {
                                    Params {
                                        is_nullable: left_node.element.is_nullable
                                            && right_node.element.is_nullable,
                                        first_pos: if left_node.element.is_nullable {
                                            utils::make_sets_union(
                                                &left_node.element.first_pos,
                                                &right_node.element.first_pos,
                                            )
                                        } else {
                                            left_node.element.first_pos.clone()
                                        },
                                        last_pos: if right_node.element.is_nullable {
                                            utils::make_sets_union(
                                                &left_node.element.last_pos,
                                                &right_node.element.last_pos,
                                            )
                                        } else {
                                            right_node.element.last_pos.clone()
                                        },
                                        follow_pos: utils::make_empty_set(),
                                    }
                                } else {
                                    Params {
                                        is_nullable: left_node.element.is_nullable
                                            || right_node.element.is_nullable,
                                        first_pos: utils::make_sets_union(
                                            &left_node.element.first_pos,
                                            &right_node.element.first_pos,
                                        ),
                                        last_pos: utils::make_sets_union(
                                            &left_node.element.last_pos,
                                            &right_node.element.last_pos,
                                        ),
                                        follow_pos: utils::make_empty_set(),
                                    }
                                };

                                trees::BinTree::from(
                                    params,
                                    trees::BinTree::NonEmpty(left_node),
                                    trees::BinTree::NonEmpty(right_node),
                                )
                            }
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
                unreachable!()
            }
        }
    }
}

fn add_follow_pos(
    params_tree: &mut trees::BinTree<Params>,
    syntax_tree: &trees::BinTree<TreeNode>,
    left_follow_pos: &types::Set<usize>,
    right_follow_pos: &types::Set<usize>,
) {
    if syntax_tree.is_empty() {
        return;
    }

    let cur_syntax_node = match syntax_tree {
        trees::BinTree::NonEmpty(tree_node) => tree_node,
        _ => unreachable!(),
    };
    let cur_params_node = match params_tree {
        trees::BinTree::NonEmpty(tree_node) => tree_node,
        _ => unreachable!(),
    };

    match cur_syntax_node.element.1 {
        Symbol::Value(_) => {
            cur_params_node.element.follow_pos = right_follow_pos.clone();
        }

        Symbol::Operator(ref operator) => {
            let left_node = match cur_params_node.left_tree {
                trees::BinTree::NonEmpty(ref node) => node,
                _ => unreachable!(),
            };

            if *operator == ops::CONCATENATION {
                let right_node = match cur_params_node.right_tree {
                    trees::BinTree::NonEmpty(ref node) => node,
                    _ => unreachable!(),
                };

                cur_params_node.element.follow_pos = utils::make_sets_union(
                    &right_node.element.first_pos,
                    &left_node.element.last_pos,
                );

                add_follow_pos(
                    &mut cur_params_node.left_tree,
                    &cur_syntax_node.left_tree,
                    &cur_params_node.element.follow_pos,
                    &cur_params_node.element.last_pos,
                );
                add_follow_pos(
                    &mut cur_params_node.right_tree,
                    &cur_syntax_node.right_tree,
                    &cur_params_node.element.follow_pos,
                    &right_follow_pos,
                );
            } else if *operator == ops::ALTERATION {
                cur_params_node.element.follow_pos = left_follow_pos.clone();

                add_follow_pos(
                    &mut cur_params_node.left_tree,
                    &cur_syntax_node.left_tree,
                    &cur_params_node.element.follow_pos,
                    &cur_params_node.element.follow_pos,
                );
                add_follow_pos(
                    &mut cur_params_node.right_tree,
                    &cur_syntax_node.right_tree,
                    &cur_params_node.element.follow_pos,
                    &cur_params_node.element.follow_pos,
                );
            } else if *operator == ops::CLOSURE {
                cur_params_node.element.follow_pos = utils::make_sets_union(
                    &left_follow_pos,
                    &utils::make_sets_union(
                        &left_node.element.first_pos,
                        &left_node.element.last_pos,
                    ),
                );

                add_follow_pos(
                    &mut cur_params_node.left_tree,
                    &cur_syntax_node.left_tree,
                    &cur_params_node.element.follow_pos,
                    &cur_params_node.element.follow_pos,
                );
            } else {
                unreachable!()
            }
        }
    };
}

pub(super) fn extract_values<'a>(
    syntax_tree: &'a trees::BinTree<TreeNode>,
    params_tree: &'a trees::BinTree<Params>,
) -> Vec<(usize, &'a vals::Value, &'a types::Set<usize>)> {
    let params_node = match params_tree {
        trees::BinTree::NonEmpty(ref node) => node,
        _ => unreachable!(),
    };

    match syntax_tree {
        trees::BinTree::NonEmpty(ref syntax_node) => match syntax_node.element.1 {
            Symbol::Value(ref value) => vec![(
                syntax_node.element.0,
                value,
                &params_node.element.follow_pos,
            )],
            Symbol::Operator(ref operator) => {
                if operator.is_unary() {
                    extract_values(&syntax_node.left_tree, &params_node.left_tree)
                } else if operator.is_binary() {
                    utils::merge_vectors(
                        extract_values(&syntax_node.left_tree, &params_node.left_tree),
                        extract_values(&syntax_node.right_tree, &params_node.right_tree),
                    )
                } else {
                    unreachable!()
                }
            }
        },
        _ => unreachable!(),
    }
}

mod tests {
    #[test]
    fn tree_new() {
        use super::*;

        AbstractSyntaxTree::new();
    }

    #[test]
    fn tree_from_str() {
        use super::*;

        let cases = vec!["((a|b)*abb)#"];

        for case in cases {
            AbstractSyntaxTree::from_str(case).unwrap();
        }
    }

    #[test]
    fn make_rpn() {
        use super::*;

        let cases = vec![
            (
                "((a|b)c|d)#",
                vec![
                    Symbol::from_value_str("a").unwrap(),
                    Symbol::from_value_str("b").unwrap(),
                    Symbol::Operator(ops::ALTERATION),
                    Symbol::from_value_str("c").unwrap(),
                    Symbol::Operator(ops::CONCATENATION),
                    Symbol::from_value_str("d").unwrap(),
                    Symbol::Operator(ops::ALTERATION),
                    Symbol::from_value_str(vals::Value::SPECIAL).unwrap(),
                    Symbol::Operator(ops::CONCATENATION),
                ]
                .into_iter()
                .collect::<types::Queue<_>>(),
            ),
            (
                "(a*b|cd)#",
                vec![
                    Symbol::from_value_str("a").unwrap(),
                    Symbol::Operator(ops::CLOSURE),
                    Symbol::from_value_str("b").unwrap(),
                    Symbol::Operator(ops::CONCATENATION),
                    Symbol::from_value_str("c").unwrap(),
                    Symbol::from_value_str("d").unwrap(),
                    Symbol::Operator(ops::CONCATENATION),
                    Symbol::Operator(ops::ALTERATION),
                    Symbol::from_value_str(vals::Value::SPECIAL).unwrap(),
                    Symbol::Operator(ops::CONCATENATION),
                ]
                .into_iter()
                .collect::<types::Queue<_>>(),
            ),
        ];

        for case in cases {
            assert_eq!(super::make_rpn(case.0).unwrap(), case.1);
        }
    }

    #[test]
    fn make_tree() {
        use super::*;

        let cases = vec![(
            "(a*b|cd)#",
            trees::BinTree::from(
                TreeNode(0, Symbol::Operator(ops::CONCATENATION)),
                trees::BinTree::from(
                    TreeNode(0, Symbol::Operator(ops::ALTERATION)),
                    trees::BinTree::from(
                        TreeNode(0, Symbol::Operator(ops::CONCATENATION)),
                        trees::BinTree::from_element_with_left(
                            TreeNode(0, Symbol::Operator(ops::CLOSURE)),
                            TreeNode(1, Symbol::from_value_str("a").unwrap()),
                        ),
                        trees::BinTree::from_element(TreeNode(
                            2,
                            Symbol::from_value_str("b").unwrap(),
                        )),
                    ),
                    trees::BinTree::from(
                        TreeNode(0, Symbol::Operator(ops::CONCATENATION)),
                        trees::BinTree::from_element(TreeNode(
                            3,
                            Symbol::from_value_str("c").unwrap(),
                        )),
                        trees::BinTree::from_element(TreeNode(
                            4,
                            Symbol::from_value_str("d").unwrap(),
                        )),
                    ),
                ),
                trees::BinTree::from_element(TreeNode(
                    5,
                    Symbol::from_value_str(vals::Value::SPECIAL).unwrap(),
                )),
            ),
        )];

        for case in cases {
            assert_eq!(
                super::make_tree(super::make_rpn(case.0).unwrap()).unwrap(),
                case.1
            );
        }
    }

    #[test]
    fn make_params_tree() {
        use super::*;

        let cases = vec![(
            "((a|b)*)#",
            trees::BinTree::from(
                Params {
                    is_nullable: false,
                    first_pos: utils::make_set_from_vec(vec![1, 2, 3]),
                    last_pos: utils::make_set_from_vec(vec![3]),
                    follow_pos: utils::make_set_from_vec(vec![1, 2, 3]),
                },
                trees::BinTree::from(
                    Params {
                        is_nullable: true,
                        first_pos: utils::make_set_from_vec(vec![1, 2]),
                        last_pos: utils::make_set_from_vec(vec![1, 2]),
                        follow_pos: utils::make_set_from_vec(vec![1, 2, 3]),
                    },
                    trees::BinTree::from(
                        Params {
                            is_nullable: false,
                            first_pos: utils::make_set_from_vec(vec![1, 2]),
                            last_pos: utils::make_set_from_vec(vec![1, 2]),
                            follow_pos: utils::make_set_from_vec(vec![1, 2, 3]),
                        },
                        trees::BinTree::from_element(Params {
                            is_nullable: false,
                            first_pos: utils::make_set_from_vec(vec![1]),
                            last_pos: utils::make_set_from_vec(vec![1]),
                            follow_pos: utils::make_set_from_vec(vec![1, 2, 3]),
                        }),
                        trees::BinTree::from_element(Params {
                            is_nullable: false,
                            first_pos: utils::make_set_from_vec(vec![2]),
                            last_pos: utils::make_set_from_vec(vec![2]),
                            follow_pos: utils::make_set_from_vec(vec![1, 2, 3]),
                        }),
                    ),
                    trees::BinTree::Empty,
                ),
                trees::BinTree::from_element(Params {
                    is_nullable: false,
                    first_pos: utils::make_set_from_vec(vec![3]),
                    last_pos: utils::make_set_from_vec(vec![3]),
                    follow_pos: utils::make_empty_set(),
                }),
            ),
        )];

        for case in cases {
            let t = AbstractSyntaxTree::from_str(case.0).unwrap();
            let p = t.make_params_tree();
            assert_eq!(p, case.1)
        }
    }

    #[test]
    fn extract_values() {
        use super::*;

        let cases = vec![(
            "((a|b)*)#",
            vec![
                (
                    1,
                    vals::Value::from_valid_str("a"),
                    utils::make_set_from_vec(vec![1, 2, 3]),
                ),
                (
                    2,
                    vals::Value::from_valid_str("b"),
                    utils::make_set_from_vec(vec![1, 2, 3]),
                ),
                (3, vals::Value::from_valid_str("#"), utils::make_empty_set()),
            ],
        )];

        for case in &cases {
            let t = AbstractSyntaxTree::from_str(case.0).unwrap();
            let p = t.make_params_tree();

            let actual = super::extract_values(t.root(), &p);
            let expected = case
                .1
                .iter()
                .map(|x| (x.0 as usize, &x.1, &x.2))
                .collect::<Vec<_>>();

            assert_eq!(actual, expected);
        }
    }
}
