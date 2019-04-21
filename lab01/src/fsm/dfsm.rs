use crate::{
    regexp,
    regexp::{ast, vals},
    trees, types,
};

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

#[derive(PartialEq, Clone, Debug)]
struct State(types::Set<usize>);

#[derive(PartialEq, Clone, Debug)]
struct Transition {
    initial_state: State,
    final_state: State,
    symbol: vals::Value,
}

fn extract_values_positions<'a>(
    syntax_tree: &'a trees::BinTree<ast::TreeNode>,
    params_tree: &'a trees::BinTree<ast::Params>,
) -> Vec<(usize, &'a vals::Value, &'a types::Set<usize>)> {
    let params_node = match params_tree {
        trees::BinTree::NonEmpty(ref node) => node,
        _ => unreachable!(),
    };

    match syntax_tree {
        trees::BinTree::NonEmpty(ref syntax_node) => match syntax_node.element.1 {
            ast::Symbol::Value(ref value) => vec![(
                syntax_node.element.0,
                value,
                &params_node.element.follow_pos,
            )],
            ast::Symbol::Operator(ref operator) => {
                if operator.is_unary() {
                    extract_values_positions(&syntax_node.left_tree, &params_node.left_tree)
                } else if operator.is_binary() {
                    merge_vectors(
                        extract_values_positions(&syntax_node.left_tree, &params_node.left_tree),
                        extract_values_positions(&syntax_node.right_tree, &params_node.right_tree),
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
    use super::*;
    use crate::utils;
    use std::str::FromStr;

    #[test]
    fn extract_values_positions() {
        let cases = vec![(
            "((a|b)*)#",
            vec![
                (
                    1,
                    vals::Value::from_valid_str("a"),
                    utils::make_set_from_vec(vec![1, 2, 5]),
                ),
                (
                    2,
                    vals::Value::from_valid_str("b"),
                    utils::make_set_from_vec(vec![1, 2, 5]),
                ),
                (5, vals::Value::from_valid_str("#"), utils::make_empty_set()),
            ],
        )];

        for case in &cases {
            let r = regexp::RegExp::from_str(case.0).unwrap();

            let t = r.syntax_tree();
            let p = r.params_tree();

            let actual = super::extract_values_positions(t.root(), p);
            let expected = case
                .1
                .iter()
                .map(|x| (x.0 as usize, &x.1, &x.2))
                .collect::<Vec<_>>();

            assert_eq!(actual, expected);
        }
    }
}
