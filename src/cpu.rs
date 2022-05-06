// SPDX-License-Identifier: Apache-2.0
// Copyright 2021 IBM Corp.
use crate::collector::*;
use lazy_regex::regex;


pub fn get_cores(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    let captures = col.parse_from_command("lscpu", regex!(r"\s+Core\(s\) per socket:\s+(\d+)"), OutputStream::STDOUT)?;
    Ok(CollectorValue::Integer(captures[1].parse::<i64>().unwrap()))
}
