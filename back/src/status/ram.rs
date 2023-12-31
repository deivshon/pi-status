use super::DOCKER_PROC_DIR_ENV;

use std::fs;

use anyhow::Result;
use lazy_static::lazy_static;
use serde::Serialize;

const PROC_MEMINFO_DEFAULT: &str = "/proc/meminfo";

lazy_static! {
    static ref PROC_MEMINFO: String = if let Ok(proc) = std::env::var(DOCKER_PROC_DIR_ENV) {
        format!("{}/meminfo", proc)
    } else {
        String::from(PROC_MEMINFO_DEFAULT)
    };
}

const TOTAL_LABEL: &str = "MemTotal:";
const FREE_LABEL: &str = "MemFree:";
const AVAILABLE_LABEL: &str = "MemAvailable:";
const CACHED_LABEL: &str = "Cached:";

const TOTAL: usize = 0;
const FREE: usize = 1;
const AVAILABLE: usize = 2;
const CACHED: usize = 3;

const LABELS: &'static [&str] = &[TOTAL_LABEL, FREE_LABEL, AVAILABLE_LABEL, CACHED_LABEL];

#[derive(Serialize)]
pub struct Ram {
    total: u64,
    used: u64,
    available: u64,
    free: u64,
    cached: u64,
}

fn get_ram() -> Result<Ram> {
    let meminfo = fs::read_to_string((*PROC_MEMINFO).as_str())?;

    let mut mem_values: Vec<u64> = vec![];
    for line in meminfo.lines() {
        let split_line = line.split_whitespace().collect::<Vec<&str>>();

        if split_line.len() < 2 || !LABELS.contains(&split_line[0]) {
            continue;
        }

        mem_values.push(split_line[1].parse::<u64>()? * 1024);
    }

    return Ok(Ram {
        total: mem_values[TOTAL],
        used: mem_values[TOTAL] - mem_values[AVAILABLE],
        available: mem_values[AVAILABLE],
        free: mem_values[FREE],
        cached: mem_values[CACHED],
    });
}

pub fn get() -> Option<Ram> {
    match get_ram() {
        Ok(ram_data) => Some(ram_data),
        Err(e) => {
            eprintln!("Error in RAM component: {}", e);
            None
        }
    }
}
