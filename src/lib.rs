use std::env;

pub struct Action {
    id: u32,
    description: String,
    pub completion: bool,
}

pub struct Config {
    pub autosave: bool,
    pub args: String,
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
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let arg = match args.next() {
            Some(ar) => ar,
            None => return Err("No arguments passed"),
        };

        let autosave = env::var("AUTOSAVE").is_ok();
        Ok(Config {
            autosave,
            args: arg,
        })
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
