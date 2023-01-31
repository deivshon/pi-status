use std::fs;
use std::sync::Mutex;

use std::fmt;

use serde::Serialize;

use anyhow::{Result, Error};

use lazy_static::lazy_static;

lazy_static! {
    static ref LAST: Mutex<Vec<CpuUsage>> = Mutex::new(vec![]);
}

const PROC_STAT: &str = "/proc/stat";

const USER: usize = 1;
const NICE: usize = 2;
const SYSTEM: usize = 3;
const IDLE: usize = 4;
const IOWAIT: usize = 5;
const IRQ: usize = 6;
const SOFTIRQ: usize = 7;
const STEAL: usize = 8;
const GUEST: usize = 9;
const GUEST_NICE: usize = 10;

#[derive(Clone, Serialize)]
pub struct CpuUsage {
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
    steal: u64,
    guest: u64,
    guest_nice: u64
}

#[derive(Debug)]
enum CpuErr {
    CoresChanged
}

impl std::error::Error for CpuErr {}

impl fmt::Display for CpuErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CpuErr::CoresChanged => write!(f, "The number of cores changed")
        }
    }
}

fn get_cpu_data() -> Result<Vec<CpuUsage>> {
    let mut last_usage = LAST.lock().unwrap();
    let first = last_usage.is_empty();

    let mut cpu_usage: Vec<CpuUsage> = vec![];
    let proc_stat = fs::read_to_string(PROC_STAT)?;
    for line in proc_stat.lines() {
        let split_line = line.split(" ").filter(|x| *x != "").collect::<Vec<&str>>();

        if split_line.len() != 11 || split_line[0] == "intr" {break}

        cpu_usage.push(CpuUsage {
            user: split_line[USER].parse::<u64>()?,
            nice: split_line[NICE].parse::<u64>()?,
            system: split_line[SYSTEM].parse::<u64>()?,
            idle: split_line[IDLE].parse::<u64>()?,
            iowait: split_line[IOWAIT].parse::<u64>()?,
            irq: split_line[IRQ].parse::<u64>()?,
            softirq: split_line[SOFTIRQ].parse::<u64>()?,
            steal: split_line[STEAL].parse::<u64>()?,
            guest: split_line[GUEST].parse::<u64>()?,
            guest_nice: split_line[GUEST_NICE].parse::<u64>()?
        });
        
        if first {
            last_usage.push(cpu_usage[cpu_usage.len() - 1].clone());
        }
        else {
            let i = cpu_usage.len() - 1;

            if last_usage.len() <= i {
                return Err(Error::new(CpuErr::CoresChanged));
            }

            let cur_cpu_usage = cpu_usage[i].clone();

            cpu_usage[i].user -= last_usage[i].user;
            cpu_usage[i].nice -= last_usage[i].nice;
            cpu_usage[i].system -= last_usage[i].system;
            cpu_usage[i].idle -= last_usage[i].idle;
            cpu_usage[i].iowait -= last_usage[i].iowait;
            cpu_usage[i].irq -= last_usage[i].irq;
            cpu_usage[i].softirq -= last_usage[i].softirq;
            cpu_usage[i].steal -= last_usage[i].steal;
            cpu_usage[i].guest -= last_usage[i].guest;
            cpu_usage[i].guest_nice -= last_usage[i].guest_nice;

            last_usage[i] = cur_cpu_usage;
        }
    }

    if last_usage.len() != cpu_usage.len() {
        return Err(Error::new(CpuErr::CoresChanged));
    }

    return Ok(cpu_usage)
}

pub fn get() -> Option<Vec<CpuUsage>> {
    match get_cpu_data() {
        Ok(usage) => Some(usage),
        Err(e) => {
            eprintln!("Error in CPU component: {}", e);
            None
        }
    }
}
