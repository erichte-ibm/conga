use crate::collector::*;
use regex::Regex;

pub fn get_cores(col: &mut Collector) -> CollectorValue {
    let re = Regex::new(r"\s+Core\(s\) per socket:\s+(\d+)").unwrap();

    let out = col.run_command("lscpu");

    let output = String::from_utf8(out.stdout).unwrap();
    let caps = re.captures(&output).unwrap();

    CollectorValue::Number(caps[1].parse::<i64>().unwrap())
}