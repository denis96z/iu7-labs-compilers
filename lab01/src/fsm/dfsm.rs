use std::collections::HashSet;

use crate::regexp::{ast, vals};
use crate::{regexp, trees};

pub struct DFSM<'a> {
    states: Vec<State>,
    valid_values: Vec<State>,
    transitions: Vec<Transition>,
    initial_state: &'a State,
    final_states: &'a [State],
}

impl<'a> DFSM<'a> {
    fn from_regexp(r: &regexp::RegExp) -> Self {
        unimplemented!()
    }
}

type Set<T> = HashSet<T>;

#[derive(PartialEq, Clone, Debug)]
struct State(Set<usize>);

#[derive(PartialEq, Clone, Debug)]
struct Transition {
    initial_state: State,
    final_state: State,
    symbol: vals::Value,
}

fn extract_values_positions(
    syntax_tree: &trees::BinTree<ast::TreeNode>,
) -> Vec<(usize, &vals::Value)> {
    match syntax_tree {
        trees::BinTree::NonEmpty(ref node) => match node.element.1 {
            ast::Symbol::Value(ref value) => vec![(node.element.0, value)],
            ast::Symbol::Operator(ref operator) => {
                if operator.is_unary() {
                    extract_values_positions(&node.left_tree)
                } else if operator.is_binary() {
                    merge_vectors(
                        extract_values_positions(&node.left_tree),
                        extract_values_positions(&node.right_tree),
                    )
                } else {
                    unreachable!()
                }
            }
        },
        _ => unreachable!(),
    }
}

fn merge_vectors<T>(mut v1: Vec<T>, mut v2: Vec<T>) -> Vec<T> {
    v1.append(&mut v2);
    return v1;
}

mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn extract_values_positions() {
        let cases = vec![(
            "((a|b)*)#",
            vec![
                (1usize, vals::Value::from_valid_str("a")),
                (2usize, vals::Value::from_valid_str("b")),
                (5usize, vals::Value::from_valid_str("#")),
            ],
        )];

        for case in &cases {
            let t = ast::AbstractSyntaxTree::from_str(case.0).unwrap();

            let actual = super::extract_values_positions(&t.root());
            let expected = case.1.iter().map(|x| (x.0, &x.1)).collect::<Vec<_>>();

            assert_eq!(actual, expected);
        }
    }
}
