use crate::regexp;
use crate::regexp::vals;

pub struct DFSM<'a> {
    states: Vec<State>,
    valid_values: Vec<State>,
    transitions: Vec<Transition>,
    initial_state: &'a State,
    final_states: &'a [State],
}

impl<'a> DFSM<'a> {
    const EMPTY_STATE: State = State(0);

    fn from_regexp(r: &regexp::RegExp) -> Self {
        unimplemented!()
    }
}

#[derive(PartialEq, Clone, Debug)]
struct State(usize);

#[derive(PartialEq, Clone, Debug)]
struct Transition {
    initial_state: State,
    final_state: State,
    symbol: vals::Value,
}
