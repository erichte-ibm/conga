// SPDX-License-Identifier: Apache-2.0
// Copyright 2021 IBM Corp.

use std::collections::HashMap;
use std::process::Command;
use std::process::Output;
use regex::Regex;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

use serde_json;

pub type CollectorErr = Box<dyn std::error::Error>;

#[allow(dead_code)]
pub enum OutputStream{
    STDOUT,
    STDERR,
}
// TODO: should we include an error type in here too to display collector function errors?
#[allow(dead_code)]
pub enum CollectorValue {
    // TODO: Perhaps we can sub-enum the numbers together?
    Integer(i64),
    Float(f64),
    Text(String),
    // probably will need more here
}

pub struct Collector {
    raw: HashMap<String, Output>, // command -> output
    // TODO: consider making this just a list?
    //  Doesn't really need to be a HashMap if we aren't referring to it later
    data: Vec<(String, CollectorValue)>, // tag -> value
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

    // returns a vector of strings for every match captured from output
    pub fn parse_from_command(&mut self, command: &str, regex: &Regex, out_type: OutputStream) -> Result<Vec<String>, CollectorErr> {
        let out = self.run_command(command);

        let output = match out_type {
            OutputStream::STDOUT => String::from_utf8(out.stdout)?,
            OutputStream::STDERR => String::from_utf8(out.stderr)?,
        };

        let cap = regex.captures(&output);
        return match cap {
            Some(c) => Ok(
                c.iter()
                    .map(| match_str | match_str.unwrap().as_str().to_string())
                    .collect()
                ),
            None => Err(format!(
                "Failed to parse output of command '{}' with regex '{}' over output:\n{}",
                command, regex, output).into()),
        }
    }

    pub fn add_data(&mut self, tag: String, data: CollectorValue) {
        self.data.push((tag, data)); // TODO: does this need a copy?
    }

    pub fn new() -> Collector {
        Collector{raw: HashMap::new(), data: Vec::new() }
    }

    pub fn to_string(self) -> String {
        let mut map = serde_json::Map::new();
        for d in self.data {
            let val = match d.1 {
                CollectorValue::Integer(n) => serde_json::Value::Number(serde_json::Number::from(n)),
                CollectorValue::Float(f) => serde_json::Value::Number(serde_json::Number::from_f64(f).unwrap()),
                CollectorValue::Text(t) => serde_json::Value::String(t),
            };

            map.insert(d.0, val);
        }

        serde_json::to_string(&map).unwrap()
    }

    pub fn print(self) {
        println!("{}", self.to_string());
    }

    pub fn write_file(self, output: &str) {
        let output = PathBuf::from(output);
        let mut file = match File::create(&output) {
            Err(x) => panic!("couldn't write to {}: {}", output.display(), x),
            Ok(f) => f,
        };

        file.write_all(self.to_string().as_bytes()).unwrap();
    }
 }
