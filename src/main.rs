use std::env;
use todo::{get_new_id, parse_and_return_args, Action};

fn main() {
    let args = parse_and_return_args(env::args());
    run(args);
}

fn run(args: Vec<String>) {
    let len = args.len();
    let flag = &args.clone()[0];
    let mut vector = Vec::new();
    match flag.as_str() {
        "n" => {
            for arg in args {
                if arg == *flag {
                    let _ = arg;
                }
                vector.push(arg);
            }
            let mut completion = true;
            if vector[2].to_lowercase() == "true" {
            } else if vector[2].to_lowercase() == "false" {
                completion = false;
            } else {
                panic!("Unknown parameter: {}", vector[2]);
            }
            let action = Action::new(0, <String as Clone>::clone(&vector[1]), completion);
            action.show();
        }
        _ => panic!("In devlopment"),
    }
}
