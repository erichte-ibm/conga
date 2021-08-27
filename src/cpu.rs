// SPDX-License-Identifier: Apache-2.0
// Copyright 2021 IBM Corp.

use crate::collector::*;
use regex::Regex;

pub fn get_cores(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    // TODO: There is a lot of boilerplate for this, can we factor this out into a simpler function?
    let re = Regex::new(r"\s+Core\(s\) per socket:\s+(\d+)").unwrap();

    let out = col.run_command("lscpu");

    // TODO: lot of panicky unwraps in here, we should make this more fault-tolerant
    let output = String::from_utf8(out.stdout).unwrap();
    let caps = re.captures(&output).unwrap();

    // TODO: maybe not assume the match worked correctly
    Ok(CollectorValue::Integer(caps[1].parse::<i64>().unwrap()))
}
