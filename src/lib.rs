use csv;
use std::{
    clone,
    error::Error,
    io,
    time::{SystemTime, UNIX_EPOCH},
};

type Record = (u32, String, String, bool, u64, Option<u64>, Option<String>);

pub struct Action {
    id: u32,
    description: String,
    urgency: String,
    completion: bool,
    timestamp: u64,
    till: Option<u64>,
    time_unit: Option<String>,
}

pub struct ActionManager {
    actions: Vec<Action>,
}

impl Action {
    pub fn new(
        id: u32,
        description: String,
        urgency: String,
        completion: bool,
        timestamp: u64,
        till: Option<u64>,
        time_unit: Option<String>,
    ) -> Self {
        Action {
            id,
            description,
            urgency,
            completion,
            timestamp,
            till,
            time_unit,
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
    pub fn get_urgency(&self) -> String {
        self.urgency.clone()
    }
    pub fn is_completion(&self) -> bool {
        self.completion
    }
    pub fn show(&self) {
        match self.till {
            Some(till) => match self.time_unit.clone() {
                Some(unit) => {
                    println!(
            "Action {} with discription and urgency {}, {} set to {} made on {} and has time limit till {}",
            self.id, self.description, self.urgency, self.completion,self.timestamp,time_in_sec(&unit, &till)
                        );
                }
                None => panic!(),
            },
            None => {
                println!(
                    "Action {} with description and urgency {}, {} set to {} ",
                    self.id, self.description, self.urgency, self.completion
                )
            }
        }
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
    pub fn remove(&mut self, id: u32) {
        let index = match self.actions.iter().position(|element| element.id == id) {
            Some(index) => index,
            None => panic!("No action with such id found"),
        };
        self.actions.remove(index);
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
                let urgency = args.next().unwrap_or_else(|| {
                    panic!("No urgency provided");
                });
                let till = args.next().unwrap_or("None".to_string());
                let time_unit = args.next().unwrap_or("None".to_string());
                let condition = args.next().unwrap_or("false".to_string());
                vector.push(flag);
                vector.push(description);
                vector.push(urgency);
                vector.push(condition);
                vector.push(get_time_stamp().to_string());
                vector.push(till);
                vector.push(time_unit);
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

            "rm" => {
                let id = args.next().unwrap_or_else(|| {
                    panic!("No id provided");
                });
                vector.push(flag);
                vector.push(id);
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
        let action = Action::new(
            record.0, record.1, record.2, record.3, record.4, record.5, record.6,
        );
        actionmanager.add(action);
    }

    Ok(actionmanager)
}

pub fn save_csv(action_manager: ActionManager) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path("todo.csv")?;

    wtr.write_record([
        "ID",
        "DESCRIPTION",
        "URGENCY",
        "STATUS",
        "TIMESTAMP",
        "TILL",
        "TIME_UNIT",
    ])?;

    for action in action_manager.actions {
        wtr.write_record([
            action.id.to_string(),
            action.description.clone(),
            action.urgency.clone(),
            action.completion.to_string(),
            action.timestamp.to_string(),
            action.till.unwrap_or_default().to_string(),
            action.time_unit.unwrap_or_default().to_string(),
        ])?;
    }
    wtr.flush()?;

    Ok(())
}

pub fn get_time_stamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn get_duration_since(earlier: &u64) -> u64 {
    get_time_stamp() - earlier
}

pub fn time_in_sec(unit: &String, num: &u64) -> u64 {
    let time: u64 = match unit.as_str() {
        "h" => (num * 60 * 60) as u64,
        "m" => (num * 60) as u64,
        "d" => (num * 24 * 60 * 60) as u64,
        "w" => (num * 7 * 24 * 60 * 60) as u64,
        _ => {
            panic!("Unexpected error")
        }
    };
    time
}
