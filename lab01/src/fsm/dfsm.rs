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
