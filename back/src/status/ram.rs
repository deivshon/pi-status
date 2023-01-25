use super::StatusFields;

use std::error::Error;
use std::fs;

use serde::Serialize;

use regex;

const PROC_MEMINFO: &str = "/proc/meminfo";

const TOTAL_LABEL: &str = "MemTotal";
const FREE_LABEL: &str = "MemFree";
const AVAILABLE_LABEL: &str = "MemAvailable";
const CACHED_LABEL: &str = "Cached";

const TOTAL: usize = 0;
const FREE: usize = 1;
const AVAILABLE: usize = 2;
const CACHED: usize = 3;

const LABELS: &'static [&str] = &[
    TOTAL_LABEL,
    FREE_LABEL,
    AVAILABLE_LABEL,
    CACHED_LABEL
];

#[derive(Serialize)]
pub struct Ram {
    total: u64,
    used: u64,
    available: u64,
    free: u64,
    cached: u64
}

fn get_ram() -> Result<Ram, Box<dyn Error>> {
    let meminfo = fs::read_to_string(PROC_MEMINFO)?;
    let mem_line_regex = regex::Regex::new(r"(: *| kB)").unwrap();

    let mut mem_values: Vec<u64> = vec![];
    for line in meminfo.lines() {
        let split_line = mem_line_regex.split(line).collect::<Vec<&str>>();
        
        if split_line.len() < 2 || !LABELS.contains(&split_line[0]) {continue}
        
        mem_values.push(split_line[1].parse::<u64>()? * 1024);
    }

    return Ok(Ram {
        total: mem_values[TOTAL],
        used: mem_values[TOTAL] - mem_values[AVAILABLE],
        available: mem_values[AVAILABLE],
        free: mem_values[FREE],
        cached: mem_values[CACHED]
    })
}

pub fn get() -> StatusFields {
    if let Ok(ram_data) = get_ram() {
        return StatusFields::Ram(Some(ram_data));
    };

    return StatusFields::Ram(None)
}
