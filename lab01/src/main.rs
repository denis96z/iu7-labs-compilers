use std::{
    env::{args, current_exe},
    process::exit,
    str::FromStr,
};

mod fsm;
mod regexp;
mod trees;
mod types;
mod utils;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    if args.len() != 1 {
        eprintln!(
            "Wrong format. Expected:\n{} [REGEXP]",
            current_exe().unwrap().to_str().unwrap()
        );
        exit(-1);
    }

    let s = &args[0];
    let r = regexp::RegExp::from_str(s).unwrap();
    let dfsm = fsm::DFSM::from(&r);

    dbg!(&dfsm);
}
