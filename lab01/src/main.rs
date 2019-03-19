use std::env;

mod fsm;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!(
            "USAGE:\n {0} regex dfsm-file-path min-dfsm-file-path",
            &args[0]
        );
        return;
    }

    let regex = args[1].clone() + "#";
    let dfsm = fsm::DFSM::from_regex(&regex);
}
