// SPDX-License-Identifier: Apache-2.0
// Copyright 2021 IBM Corp.
use crate::collector::*;

pub fn get_cores(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    match col.get_regex_from_command("lscpu", r"\s+Core\(s\) per socket:\s+(\d+)") {
        None => Err("Failed to find regex capture".into()),
        Some(capture) => Ok(CollectorValue::Integer(capture.parse::<i64>().unwrap()))
    }
}
