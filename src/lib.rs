use csv;
use std::{error::Error, io};

type Record = (u32, String, bool);

pub struct Action {
    id: u32,
    description: String,
    completion: bool,
}

pub struct ActionManager {
    actions: Vec<Action>,
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

impl ActionManager {
    pub fn new(actions: Vec<Action>) -> ActionManager {
        ActionManager { actions }
    }
    pub fn add(&mut self, action: Action) {
        self.actions.push(action);
    }
    pub fn set_status(&mut self, id: u32, status: bool) -> Result<(), &'static str> {
        for action in &mut self.actions {
            if action.get_id() == id {
                action.completion = status;
                return Ok(());
            }
        }
        return Err("No action found with id {id}");
    }
    pub fn show(&self) {
        for action in &self.actions {
            action.show();
        }
    }
}

pub fn get_new_id(actions: &ActionManager) -> u32 {
    let mut num = 0;
    for action in &actions.actions {
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

            "s" => {
                let id = args.next().unwrap_or_else(|| {
                    panic!("No id provided");
                });
                let condition = args.next().unwrap_or("false".to_string());
                vector.push(flag);
                vector.push(id);
                vector.push(condition);
            }
            _ => panic!("In devlopment"),
        }
    }

    vector
}

pub fn read_csv() -> Result<ActionManager, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut actionmanager = ActionManager::new(Vec::new());

    for result in rdr.deserialize() {
        let record: Record = result?;
        let action = Action::new(record.0, record.1, record.2);
        actionmanager.add(action);
    }

    Ok(actionmanager)
}

pub fn save_csv(action_manager: ActionManager) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path("todo.csv")?;

    wtr.write_record(["ID", "DESCRIPTION", "STATUS"])?;

    for action in action_manager.actions {
        wtr.write_record([
            action.id.to_string(),
            action.description.clone(),
            action.completion.to_string(),
        ])?;
    }
    wtr.flush()?;

    Ok(())
}
