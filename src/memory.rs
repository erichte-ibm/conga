// SPDX-License-Identifier: Apache-2.0
// Copyright 2021 IBM Corp.

use crate::collector::*;
use regex::Regex;

// add -h to get human readable sizes, else in bytes
const MEM_COMMAND: &'static str = "free";


pub fn get_mem_total(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    let re = Regex::new(r"\s+Mem:\s+(\d+)").unwrap();
    let out = col.run_command(MEM_COMMAND);

    let output = String::from_utf8(out.stdout).unwrap();
    let caps = re.captures(&output).unwrap();

    Ok(CollectorValue::Integer(caps[1].parse::<i64>().unwrap()))
}

pub fn get_mem_used(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    let re = Regex::new(r"\s+Mem:\s+\d+\s+(\d+)").unwrap();
    let out = col.run_command(MEM_COMMAND);

    let output = String::from_utf8(out.stdout).unwrap();
    let caps = re.captures(&output).unwrap();

    Ok(CollectorValue::Integer(caps[1].parse::<i64>().unwrap()))
}

pub fn get_mem_free(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    let re = Regex::new(r"\s+Mem:\s+(?:\d+\s+){2}(\d+)").unwrap();
    let out = col.run_command(MEM_COMMAND);

    let output = String::from_utf8(out.stdout).unwrap();
    let caps = re.captures(&output).unwrap();

    Ok(CollectorValue::Integer(caps[1].parse::<i64>().unwrap()))
}

pub fn get_mem_shared(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    let re = Regex::new(r"\s+Mem:\s+(?:\d+\s+){3}(\d+)").unwrap();
    let out = col.run_command(MEM_COMMAND);

    let output = String::from_utf8(out.stdout).unwrap();
    let caps = re.captures(&output).unwrap();

    Ok(CollectorValue::Integer(caps[1].parse::<i64>().unwrap()))
}

pub fn get_mem_buff_and_cache(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    let re = Regex::new(r"\s+Mem:\s+(?:\d+\s+){4}(\d+)").unwrap();
    let out = col.run_command(MEM_COMMAND);

    let output = String::from_utf8(out.stdout).unwrap();
    let caps = re.captures(&output).unwrap();

    Ok(CollectorValue::Integer(caps[1].parse::<i64>().unwrap()))
}

pub fn get_mem_available(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    let re = Regex::new(r"\s+Mem:\s+(?:\d+\s+){5}(\d+)").unwrap();
    let out = col.run_command(MEM_COMMAND);

    let output = String::from_utf8(out.stdout).unwrap();
    let caps = re.captures(&output).unwrap();

    Ok(CollectorValue::Integer(caps[1].parse::<i64>().unwrap()))
}

// Do similar logic for swap values

pub fn get_swap_total(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    let re = Regex::new(r"\s+Swap:\s+(\d+)").unwrap();
    let out = col.run_command(MEM_COMMAND);

    let output = String::from_utf8(out.stdout).unwrap();
    let caps = re.captures(&output).unwrap();

    Ok(CollectorValue::Integer(caps[1].parse::<i64>().unwrap()))
}

pub fn get_swap_used(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    let re = Regex::new(r"\s+Swap:\s+\d+\s+(\d+)").unwrap();
    let out = col.run_command(MEM_COMMAND);

    let output = String::from_utf8(out.stdout).unwrap();
    let caps = re.captures(&output).unwrap();

    Ok(CollectorValue::Integer(caps[1].parse::<i64>().unwrap()))
}

pub fn get_swap_free(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    let re = Regex::new(r"\s+Swap:\s+(?:\d+\s+){2}(\d+)").unwrap();
    let out = col.run_command(MEM_COMMAND);

    let output = String::from_utf8(out.stdout).unwrap();
    let caps = re.captures(&output).unwrap();

    Ok(CollectorValue::Integer(caps[1].parse::<i64>().unwrap()))
}