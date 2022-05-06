// SPDX-License-Identifier: Apache-2.0
// Copyright 2021 IBM Corp.

use crate::collector::*;
use lazy_regex::regex;

const KERNEL_RELEASE_COMMAND: &'static str = "uname --kernel-release";
const OS_NAME_COMMAND: &'static str = "grep PRETTY_NAME /etc/os-release";

pub fn get_kernel_release(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    let captures = col.parse_from_command(KERNEL_RELEASE_COMMAND, regex!(r"(.*)"), OutputStream::STDOUT)?;
    Ok(CollectorValue::Text(captures[1].to_string()))
}


pub fn get_os_name(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    let captures = col.parse_from_command(OS_NAME_COMMAND, regex!(r"PRETTY_NAME=.(.*)."), OutputStream::STDOUT)?;
        Ok(CollectorValue::Text(captures[1].to_string()))
}
