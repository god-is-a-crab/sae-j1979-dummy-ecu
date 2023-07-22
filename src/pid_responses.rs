use anyhow;
use serde_yaml;
use std::collections::HashMap;
use std::fs::File;

static FILE: &str = "pid_responses.yaml";

pub fn parse_pid_responses() -> anyhow::Result<HashMap<u8, Vec<u8>>> {
    let reader = File::open(FILE)?;
    let yaml: HashMap<u8, Vec<u8>> = serde_yaml::from_reader(reader)?;
    Ok(yaml)
}
