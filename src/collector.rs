use std::collections::HashMap;
use std::process::Command;
use std::process::Output;

// TODO: should we include an error type in here too to display collector function errors?
#[allow(dead_code)]
pub enum CollectorValue {
    Number(i64),
    Text(String),
    // probably will need more here
}

pub struct Collector {
    raw: HashMap<String, Output>, // command -> output
    // TODO: consider making this just a list?
    //  Doesn't really need to be a HashMap if we aren't referring to it later
    data: HashMap<String, CollectorValue>, // tag -> value
}


impl Collector {
    // TODO: Maybe lifetime this?
    pub fn run_command(&mut self, command: &str) -> Output {
        let tmp = self.raw.get(command);
        if tmp.is_some() {
            return tmp.unwrap().clone();
        }

        // TODO: multiple spaces between args?
        let args: Vec<&str> = command.split(" ").collect();

        let mut com = Command::new(args[0]);
        for i in &args[1..] {
            com.arg(i);
        }

        // TODO: Clarify here probably, specify command, etc
        // Something very very wrong happened here running a command
        let output = com.output().unwrap();

        self.raw.insert(String::from(command), output);
        self.raw.get(command).unwrap().clone()
    }

    pub fn add_data(&mut self, tag: String, data: CollectorValue) {
        self.data.insert(tag, data); // TODO: does this need a copy?
    }

    pub fn new() -> Collector {
        Collector{raw: HashMap::new(), data: HashMap::new() }
    }
}
