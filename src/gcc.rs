// SPDX-License-Identifier: Apache-2.0
// Copyright 2021 IBM Corp.

use crate::collector::*;
use lazy_regex::regex;

const GCC_COMMAND: &'static str = "gcc -v";

pub fn version(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    let captures = col.parse_from_command(GCC_COMMAND, regex!(r"gcc version (\d.*)"), OutputStream::STDERR)?;
    Ok(CollectorValue::Text(captures[1].to_string()))
}

pub fn flags(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    let captures = col.parse_from_command(GCC_COMMAND, regex!(r"Configured with: .*?configure (.*)"), OutputStream::STDERR)?;
    Ok(CollectorValue::Text(captures[1].to_string()))
}
