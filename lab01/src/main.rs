#[macro_use]
extern crate log;

mod fsm;
mod regexp;

use fsm::dfsm;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!(
            "USAGE:\n {0} regex dfsm-file-path min-dfsm-file-path",
            &args[0]
        );
        return;
    }

    let regex = args[1].clone() + "#";
    dbg!(&regex);

    let dfsm = dfsm::DFSM::from_regex(&regex).unwrap();
}
