use std::env;
use todo::{get_new_id, parse_and_return_args, read_csv, save_csv, Action};

fn main() {
    let args = parse_and_return_args(env::args());
    run(args);
}

fn run(args: Vec<String>) {
    let mut actionmanager = match read_csv() {
        Ok(actionmanager) => actionmanager,
        Err(err) => panic!("Got error {err}"),
    };
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
            if vector[3].to_lowercase() == "true" {
            } else if vector[3].to_lowercase() == "false" {
                completion = false;
            } else {
                panic!("Unknown parameter: {}", vector[3]);
            }
            let action = Action::new(
                get_new_id(&actionmanager),
                <String as Clone>::clone(&vector[1]),
                <String as Clone>::clone(&vector[2]),
                completion,
            );
            actionmanager.add(action);
            actionmanager.show();
        }
        "s" => {
            for arg in args {
                if arg == *flag {
                    let _ = arg;
                }
                vector.push(arg);
            }
            let id: u32 = vector[1].parse().unwrap();
            let mut completion = true;
            if vector[2].to_lowercase() == "true" {
                completion = true;
            } else if vector[2].to_lowercase() != "false" {
                panic!("Unrecognized parameter: {}", vector[2]);
            }
            actionmanager
                .set_status(id, completion)
                .unwrap_or_else(|err| {
                    panic!("Encoutered error {err}");
                });
            actionmanager.show();
        }
        "rm" => {
            let id: u32 = args.clone()[1].parse().unwrap();
            actionmanager.remove(id);
        }
        _ => panic!("In devlopment"),
    }
    save_csv(actionmanager).unwrap();
}
