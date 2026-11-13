use std::env;

pub struct Action {
    id: u32,
    description: String,
    pub completion: bool,
}

impl Action {
    pub fn new(id: u32, description: String, completion: bool) -> Self {
        Action {
            id,
            description,
            completion,
        }
    }
    pub fn get_description(&self) -> String {
        self.description.clone()
    }
    pub fn change_status(&mut self, status: bool) {
        self.completion = status;
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn is_completion(&self) -> bool {
        self.completion
    }
    pub fn show(&self) {
        println!(
            "Action {} with discription '{}' set to {}",
            self.id, self.description, self.completion
        );
    }
}

pub fn get_new_id(actions: Vec<Action>) -> u32 {
    let mut num = 0;
    for action in actions {
        if num == action.get_id() {
            num += 1;
        }
    }
    num
}
pub fn parse_and_return_args(mut args: impl Iterator<Item = String>) -> Vec<String> {
    args.next();
    let mut vector: Vec<String> = Vec::new();
    while let Some(flag) = args.next() {
        match flag.as_str() {
            "n" => {
                let description = args.next().unwrap_or_else(|| {
                    panic!("No description provided");
                });
                let condition = args.next().unwrap_or("false".to_string());
                vector.push(flag);
                vector.push(description);
                vector.push(condition);
            }
            _ => panic!("In devlopment"),
        }
    }

    vector
}
