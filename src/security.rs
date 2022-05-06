// SPDX-License-Identifier: Apache-2.0
// Copyright 2022 IBM Corp.
use crate::collector::*;

static BASE_DIR: &str = "/sys/devices/system/cpu/vulnerabilities/";

pub fn get_vulnerabilities(_col: &mut Collector) -> Result<Vec<(String, CollectorValue)>, CollectorErr> {
    let mut ret = Vec::new();

    // TODO: perhaps report missing directory (older kernels?) better here
    let paths = std::fs::read_dir(BASE_DIR)?;

    for p in paths {
        // TODO: perhaps skip over errors here in allow-failures mode?
        let p = p?;

        let out = _col.run_command(format!("cat {}", p.path().display()).as_str());

        let tag = String::from(p.file_name().to_str().unwrap());
        let value = CollectorValue::Text(String::from_utf8(out.stdout).unwrap().trim().to_string());

        ret.push((tag, value));
    }

    Ok(ret)
}
