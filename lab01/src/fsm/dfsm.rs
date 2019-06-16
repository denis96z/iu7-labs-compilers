use crate::{
    regexp::{self, vals},
    trees, types, utils,
};

#[derive(PartialEq, Debug)]
pub struct DFSM {
    states: Vec<usize>,
    valid_symbols: Vec<types::Symbol>,
    transitions: Vec<Transition>,
    initial_state: usize,
    final_states: Vec<usize>,
}

impl DFSM {
    pub fn minimize(&mut self) {
        let non_final_states = utils::find_diff(&self.states, &self.final_states);
        let mut state_classes = vec![self.final_states.clone(), non_final_states.clone()];

        let mut queue = types::Queue::new();
        for symbol in &self.valid_symbols {
            queue.push_back((self.final_states.clone(), symbol.clone()));
            queue.push_back((non_final_states.clone(), symbol.clone()));
        }

        while !queue.is_empty() {
            let p = queue.pop_front().unwrap();

            let mut new_queue = types::Queue::new();
            let mut new_state_classes = Vec::new();

            for r in &state_classes {
                let (r1, r2) = self.split(r, &p);
                if !r1.is_empty() && !r2.is_empty() {
                    new_state_classes.clear();
                    for c in &state_classes {
                        if *c == *r {
                            new_state_classes.push(r1.clone());
                            new_state_classes.push(r2.clone());
                        } else {
                            new_state_classes.push(r.clone());
                        }
                    }

                    let mut f = false;
                    for item in &queue {
                        if *item == (r.clone(), p.1.clone()) {
                            f = true;
                            new_queue.push_back((r1.clone(), p.1.clone()));
                            new_queue.push_back((r2.clone(), p.1.clone()));
                        } else {
                            new_queue.push_back((*item).clone());
                        }
                    }

                    if !f {
                        if r1.len() <= r2.len() {
                            new_queue.push_back((r1.clone(), p.1.clone()));
                        } else {
                            new_queue.push_back((r2.clone(), p.1.clone()));
                        }
                    }
                } else {
                    new_state_classes = state_classes.clone();
                }
            }

            queue = new_queue;
            state_classes = new_state_classes;
        }

        dbg!(&state_classes);
    }

    fn split(
        &self,
        class: &Vec<usize>,
        p: &(Vec<usize>, types::Symbol),
    ) -> (Vec<usize>, Vec<usize>) {
        dbg!(&class);
        dbg!(&p);

        let mut c1 = Vec::new();
        let mut c2 = Vec::new();

        for state in class {
            for t in &self.transitions {
                if t.initial_state == *state && t.symbol == p.1 {
                    if p.0.contains(&t.final_state) {
                        c1.push(*state);
                    } else {
                        c2.push(*state);
                    }
                }
            }
        }

        dbg!(&c1);
        dbg!(&c2);

        (c1, c2)
    }
}

impl From<&regexp::RegExp> for DFSM {
    fn from(r: &regexp::RegExp) -> Self {
        let values = r.extract_values();

        let mut valid_symbols = values
            .iter()
            .filter(|(_, value, _)| value.symbol() != vals::Value::SPECIAL)
            .map(|(_, value, _)| value.symbol().clone())
            .collect::<types::Set<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        valid_symbols.sort();

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
                    initial_state: unmarked_index,
                    final_state: new_index,
                    symbol: symbol.to_string(),
                });
            }
        }

        let special_index = values
            .iter()
            .find(|(_, value, _)| value.symbol() == vals::Value::SPECIAL)
            .map(|(index, _, _)| *index)
            .unwrap();

        let initial_state = 0;

        let final_states = states_with_marks
            .iter()
            .enumerate()
            .filter(|(_, (_, positions))| positions.contains(&special_index))
            .map(|(index, _)| index)
            .collect::<Vec<_>>();

        let states = (0..=states_with_marks.len()).collect::<Vec<_>>();
        let err_state = states.last().unwrap();

        for symbol in &valid_symbols {
            for index in 0..(states.len() - 1) {
                if transitions
                    .iter()
                    .find(|transition| {
                        transition.symbol == *symbol && transition.initial_state == index
                    })
                    .is_none()
                {
                    transitions.push(Transition {
                        initial_state: index,
                        final_state: *err_state,
                        symbol: symbol.clone(),
                    })
                }
            }
        }

        DFSM {
            states,
            valid_symbols,
            transitions,
            initial_state,
            final_states,
        }
    }
}

type State = types::Set<usize>;

#[derive(PartialEq, Clone, Debug)]
struct Transition {
    initial_state: usize,
    final_state: usize,
    symbol: types::Symbol,
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use std::str::FromStr;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn from_regexp() {
        let cases = vec![(
            "(a|b)*abb",
            DFSM {
                states: (0..=4).collect::<Vec<_>>(),
                valid_symbols: vec!["a".to_string(), "b".to_string()],
                transitions: vec![
                    Transition {
                        initial_state: 0,
                        final_state: 1,
                        symbol: "a".to_string(),
                    },
                    Transition {
                        initial_state: 0,
                        final_state: 0,
                        symbol: "b".to_string(),
                    },
                    Transition {
                        initial_state: 1,
                        final_state: 1,
                        symbol: "a".to_string(),
                    },
                    Transition {
                        initial_state: 1,
                        final_state: 2,
                        symbol: "b".to_string(),
                    },
                    Transition {
                        initial_state: 2,
                        final_state: 1,
                        symbol: "a".to_string(),
                    },
                    Transition {
                        initial_state: 2,
                        final_state: 3,
                        symbol: "b".to_string(),
                    },
                    Transition {
                        initial_state: 3,
                        final_state: 1,
                        symbol: "a".to_string(),
                    },
                    Transition {
                        initial_state: 3,
                        final_state: 0,
                        symbol: "b".to_string(),
                    },
                ],
                initial_state: 0,
                final_states: vec![3],
            },
        )];

        for (r, mut m) in cases {
            assert_eq!(DFSM::from(&regexp::RegExp::from_str(r).unwrap()), m);
            m.minimize();
        }
    }
}
