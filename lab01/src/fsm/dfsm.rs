use crate::utils::make_sets_union;
use crate::{regexp, trees, types};
use core::borrow::BorrowMut;

#[derive(Debug)]
pub struct DFSM {
    states: Vec<State>,
    valid_symbols: Vec<types::Symbol>,
    transitions: Vec<Transition>,
    initial_state_index: usize,
    final_states_indexes: Vec<usize>,
}

impl From<&regexp::RegExp> for DFSM {
    fn from(r: &regexp::RegExp) -> Self {
        let values = r.extract_values();

        let initial_state = match r.params_tree() {
            trees::BinTree::NonEmpty(ref node) => node.element.first_pos.clone(),
            _ => unreachable!(),
        };

        let valid_symbols = (b'a'..=b'z') //TODO remove hardcode
            .map(|b| char::from(b).to_string())
            .collect::<Vec<_>>();

        let mut states = Vec::new();
        states.push((false, initial_state));

        let mut transitions = Vec::new();
        loop {
            let unmarked_index = match states.iter().enumerate().find(|(_, (marked, _))| !*marked) {
                Some((index, _)) => index,
                _ => break,
            };
            states[unmarked_index].0 = true;

            for value in &values {
                let mut union = types::Set::new();
                for state in states.iter() {
                    for p in state.1.iter() {
                        if *p == value.0 {
                            union = make_sets_union(&union, value.2);
                        }
                    }
                }

                if union.is_empty() {
                    continue;
                }

                let new_index = match states
                    .iter()
                    .enumerate()
                    .find(|(_, (_, state))| *state == union)
                {
                    Some((index, _)) => index,
                    _ => states.len(),
                };
                if new_index == states.len() {
                    states.push((false, union));
                }

                transitions.push(Transition {
                    initial_state_index: unmarked_index,
                    final_state_index: new_index,
                    symbol: value.1.symbol().to_string(), //TODO optimize
                });
            }
        }

        DFSM {
            valid_symbols,
            transitions,
            states: states.into_iter().map(|(_, state)| state).collect(),
            initial_state_index: 0,
            final_states_indexes: vec![],
        }
    }
}

type State = types::Set<usize>;

#[derive(PartialEq, Clone, Debug)]
struct Transition {
    initial_state_index: usize,
    final_state_index: usize,
    symbol: types::Symbol,
}

mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn from_regexp() {
        let r = regexp::RegExp::from_str("(a|b)*a").unwrap();
        let m = DFSM::from(&r);
        dbg!(&m);
    }
}
