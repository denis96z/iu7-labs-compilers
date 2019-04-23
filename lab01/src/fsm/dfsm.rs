use crate::{
    regexp::{self, vals},
    trees, types, utils,
};

#[derive(PartialEq, Debug)]
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

        let valid_symbols = (b'a'..=b'z') //TODO
            .map(|b| char::from(b).to_string())
            .map(|s| {
                (
                    values.iter().find(|(_, v, _)| *v.symbol() == s).is_some(),
                    s,
                )
            })
            .filter(|(flag, _)| *flag)
            .map(|(_, s)| s)
            .collect::<Vec<_>>();

        let mut states_with_marks = Vec::new();
        states_with_marks.push((
            false,
            match r.params_tree() {
                trees::BinTree::NonEmpty(ref node) => node.element.first_pos.clone(),
                _ => unreachable!(),
            },
        ));

        let mut transitions = Vec::new();
        loop {
            let unmarked_index = match states_with_marks
                .iter()
                .enumerate()
                .find(|(_, (marked, _))| !*marked)
            {
                Some((index, _)) => index,
                _ => break,
            };
            states_with_marks[unmarked_index].0 = true;

            for symbol in &valid_symbols {
                let mut union = types::Set::new();
                for position in &states_with_marks[unmarked_index].1 {
                    for (index, value, follow_pos) in &values {
                        if *index != *position || *value.symbol() != *symbol {
                            continue;
                        }

                        union = utils::make_sets_union(&union, *follow_pos);
                    }
                }

                if union.is_empty() {
                    continue;
                }

                let new_index = match states_with_marks
                    .iter()
                    .enumerate()
                    .find(|(_, (_, state))| *state == union)
                {
                    Some((index, _)) => index,
                    _ => states_with_marks.len(),
                };

                if new_index == states_with_marks.len() {
                    states_with_marks.push((false, union));
                }

                transitions.push(Transition {
                    initial_state_index: unmarked_index,
                    final_state_index: new_index,
                    symbol: symbol.to_string(),
                });
            }
        }

        let special_index = values
            .iter()
            .find(|(_, value, _)| value.symbol() == vals::Value::SPECIAL)
            .map(|(index, _, _)| *index)
            .unwrap();

        let initial_state_index = 0;

        let final_states_indexes = states_with_marks
            .iter()
            .enumerate()
            .filter(|(_, (_, positions))| positions.contains(&special_index))
            .map(|(index, _)| index)
            .collect::<Vec<_>>();

        let mut states = states_with_marks
            .into_iter()
            .map(|(_, state)| state)
            .collect::<Vec<_>>();

        let err_state_index = values.len() + 1;
        states.push(utils::make_set_from_vec(vec![err_state_index]));

        for symbol in &valid_symbols {
            for index in 0..(states.len() - 1) {
                if transitions
                    .iter()
                    .find(|transition| {
                        transition.symbol == *symbol && transition.initial_state_index == index
                    })
                    .is_none()
                {
                    transitions.push(Transition {
                        initial_state_index: index,
                        final_state_index: err_state_index,
                        symbol: symbol.clone(),
                    })
                }
            }
        }

        DFSM {
            states,
            valid_symbols,
            transitions,
            initial_state_index,
            final_states_indexes,
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
    use std::str::FromStr;

    use super::*;

    #[test]
    fn from_regexp() {
        let r = regexp::RegExp::from_str("(a|b)*abb").unwrap();
        let m = DFSM::from(&r);
        dbg!(&m);
    }
}
