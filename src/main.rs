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
            let timestamp: u64 = vector[4].parse().unwrap();
            let mut till: Option<u64> = Some(1);
            if vector[5].to_lowercase() == "none" {
                till = None;
            } else {
                till = Some(vector[5].parse().unwrap());
            }
            let mut time_unit: Option<String> = Some("stuff".to_string());
            if vector[6].to_lowercase() == "none" {
                time_unit = None;
            } else {
                time_unit = Some(vector[6].clone());
            }
            let action = Action::new(
                get_new_id(&actionmanager),
                <String as Clone>::clone(&vector[1]),
                <String as Clone>::clone(&vector[2]),
                completion,
                timestamp,
                till,
                time_unit,
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
