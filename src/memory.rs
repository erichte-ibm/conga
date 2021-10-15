// SPDX-License-Identifier: Apache-2.0
// Copyright 2021 IBM Corp.

use crate::collector::*;

// add -h to get human readable sizes, else in bytes
const MEM_COMMAND: &'static str = "free";


pub fn get_mem_total(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    match col.get_regex_from_command(MEM_COMMAND, r"\s+Mem:\s+(\d+)") {
        None => Err("Failed to find regex capture".into()),
        Some(capture) => Ok(CollectorValue::Integer(capture.parse::<i64>().unwrap()))
    }
}

pub fn get_mem_used(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    match col.get_regex_from_command(MEM_COMMAND,r"\s+Mem:\s+\d+\s+(\d+)") {
        None => Err("Failed to find regex capture".into()),
        Some(capture) => Ok(CollectorValue::Integer(capture.parse::<i64>().unwrap()))
    }
}

pub fn get_mem_free(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    match col.get_regex_from_command(MEM_COMMAND, r"\s+Mem:\s+(?:\d+\s+){2}(\d+)") {
        None => Err("Failed to find regex capture".into()),
        Some(capture) => Ok(CollectorValue::Integer(capture.parse::<i64>().unwrap()))
    }
}

pub fn get_mem_shared(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    match col.get_regex_from_command(MEM_COMMAND, r"\s+Mem:\s+(?:\d+\s+){3}(\d+)") {
        None => Err("Failed to find regex capture".into()),
        Some(capture) => Ok(CollectorValue::Integer(capture.parse::<i64>().unwrap()))
    }
}

pub fn get_mem_buff_and_cache(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    match col.get_regex_from_command(MEM_COMMAND, r"\s+Mem:\s+(?:\d+\s+){4}(\d+)") {
        None => Err("Failed to find regex capture".into()),
        Some(capture) => Ok(CollectorValue::Integer(capture.parse::<i64>().unwrap()))
    }
}

pub fn get_mem_available(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    match col.get_regex_from_command(MEM_COMMAND, r"\s+Mem:\s+(?:\d+\s+){5}(\d+)") {
        None => Err("Failed to find regex capture".into()),
        Some(capture) => Ok(CollectorValue::Integer(capture.parse::<i64>().unwrap()))
    }
}

// Do similar logic for swap values

pub fn get_swap_total(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    match col.get_regex_from_command(MEM_COMMAND, r"\s+Swap:\s+(\d+)") {
        None => Err("Failed to find regex capture".into()),
        Some(capture) => Ok(CollectorValue::Integer(capture.parse::<i64>().unwrap()))
    }
}

pub fn get_swap_used(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    match col.get_regex_from_command(MEM_COMMAND, r"\s+Swap:\s+\d+\s+(\d+)") {
        None => Err("Failed to find regex capture".into()),
        Some(capture) => Ok(CollectorValue::Integer(capture.parse::<i64>().unwrap()))
    }
}

pub fn get_swap_free(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    match col.get_regex_from_command(MEM_COMMAND, r"\s+Swap:\s+(?:\d+\s+){2}(\d+)") {
        None => Err("Failed to find regex capture".into()),
        Some(capture) => Ok(CollectorValue::Integer(capture.parse::<i64>().unwrap()))
    }
}