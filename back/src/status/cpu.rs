mod consts;
pub mod err;

use std::fs;

use anyhow::{Error, Result};
use serde::Serialize;

use self::consts::{
    GUEST, GUEST_NICE, IDLE, IOWAIT, IRQ, NICE, PROC_STAT, SOFTIRQ, STEAL, SYSTEM, USER,
};
use self::err::CpuErr;

#[derive(Clone, Serialize)]
pub struct CoreUsage {
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
    steal: u64,
    guest: u64,
    guest_nice: u64,
}

#[derive(Serialize)]
pub struct CpuUsage {
    pub usage: Vec<CoreUsage>,
    last: Vec<CoreUsage>,
}

impl CpuUsage {
    pub fn new() -> Self {
        return CpuUsage {
            usage: Vec::new(),
            last: Vec::new(),
        };
    }

    pub fn update(&mut self) -> Result<()> {
        let first = self.last.is_empty();

        let mut cores_usage: Vec<CoreUsage> = vec![];
        let proc_stat = fs::read_to_string((*PROC_STAT).as_str())?;
        for line in proc_stat.lines() {
            let split_line = line.split(" ").filter(|x| *x != "").collect::<Vec<&str>>();

            if split_line.len() != 11 || split_line[0] == "intr" {
                break;
            }

            cores_usage.push(CoreUsage {
                user: split_line[USER].parse::<u64>()?,
                nice: split_line[NICE].parse::<u64>()?,
                system: split_line[SYSTEM].parse::<u64>()?,
                idle: split_line[IDLE].parse::<u64>()?,
                iowait: split_line[IOWAIT].parse::<u64>()?,
                irq: split_line[IRQ].parse::<u64>()?,
                softirq: split_line[SOFTIRQ].parse::<u64>()?,
                steal: split_line[STEAL].parse::<u64>()?,
                guest: split_line[GUEST].parse::<u64>()?,
                guest_nice: split_line[GUEST_NICE].parse::<u64>()?,
            });

            if first {
                self.last.push(cores_usage[cores_usage.len() - 1].clone());
            } else {
                let i = cores_usage.len() - 1;

                if self.last.len() <= i {
                    return Err(Error::new(CpuErr::CoresChanged));
                }

                let cur_cpu_usage = cores_usage[i].clone();

                cores_usage[i].user -= self.last[i].user;
                cores_usage[i].nice -= self.last[i].nice;
                cores_usage[i].system -= self.last[i].system;
                cores_usage[i].idle -= self.last[i].idle;
                cores_usage[i].iowait -= self.last[i].iowait;
                cores_usage[i].irq -= self.last[i].irq;
                cores_usage[i].softirq -= self.last[i].softirq;
                cores_usage[i].steal -= self.last[i].steal;
                cores_usage[i].guest -= self.last[i].guest;
                cores_usage[i].guest_nice -= self.last[i].guest_nice;

                self.last[i] = cur_cpu_usage;
            }
        }

        if self.last.len() != cores_usage.len() {
            return Err(Error::new(CpuErr::CoresChanged));
        }

        self.usage = cores_usage;
        Ok(())
    }
}
