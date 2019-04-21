use crate::{regexp, types};

#[derive(Debug)]
pub struct DFSM<'a> {
    states: Vec<State>,
    valid_values: Vec<types::Symbol>,
    transitions: Vec<Transition>,
    initial_state: &'a State,
    final_states: &'a [State],
}

impl<'a> From<&'a regexp::RegExp> for DFSM<'a> {
    fn from(_: &'a regexp::RegExp) -> Self {
        unimplemented!()
    }
}

#[derive(PartialEq, Clone, Debug)]
struct State(types::Set<usize>);

#[derive(PartialEq, Clone, Debug)]
struct Transition {
    initial_state: State,
    final_state: State,
    symbol: types::Symbol,
}
