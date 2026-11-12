use std::{env, io, process};
use todo::{get_new_id, Action, Config};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Encountered error while parrsing arguments: {err}");
        process::exit(1);
    });
    let one = run(config);
    println!(
        "{} is set to {} which has id {}",
        one.get_description(),
        one.get_id(),
        one.completion
    );
}

fn run(config: Config) -> Action {
    if config.args == String::from("n") {
        println!("Enter action description");
        let mut description = String::new();
        io::stdin().read_line(&mut description).unwrap();
        let id = 0;
        println!("Enter status (y/n)");
        let mut status = String::new();
        io::stdin().read_line(&mut status).unwrap();
        let mut completion: bool = false;
        if status.to_lowercase() == String::from("y") {
            completion = true;
        }
        return Action::new(id, description, completion);
    } else {
        panic!("Not yet made");
    }
}
