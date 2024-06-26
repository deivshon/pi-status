mod consts;
pub mod err;

use crate::status::ram::err::MemRetrievalErr;

use std::fs;

use anyhow::{Error, Result};
use serde::Serialize;

use self::consts::{AVAILABLE, CACHED, EXPECTED_MEM_VALUES, FREE, LABELS, PROC_MEMINFO, TOTAL};

#[derive(Serialize)]
pub struct RamData {
    total: u64,
    used: u64,
    available: u64,
    free: u64,
    cached: u64,
}

impl RamData {
    pub fn get() -> Result<Self> {
        let meminfo = fs::read_to_string((*PROC_MEMINFO).as_str())?;

        let mut mem_values: Vec<u64> = Vec::new();
        for line in meminfo.lines() {
            let split_line = line.split_whitespace().collect::<Vec<&str>>();

            if split_line.len() < 2 || !LABELS.contains(&split_line[0]) {
                continue;
            }

            mem_values.push(split_line[1].parse::<u64>()? * 1024);
        }

        if mem_values.len() != EXPECTED_MEM_VALUES {
            return Err(Error::new(MemRetrievalErr::NotEnoughValues));
        }
        return Ok(Self {
            total: mem_values[TOTAL],
            used: mem_values[TOTAL] - mem_values[AVAILABLE],
            available: mem_values[AVAILABLE],
            free: mem_values[FREE],
            cached: mem_values[CACHED],
        });
    }
}
