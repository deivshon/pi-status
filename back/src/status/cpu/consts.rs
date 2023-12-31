use crate::status::DOCKER_PROC_DIR_ENV;

use lazy_static::lazy_static;

const PROC_STAT_DEFAULT: &str = "/proc/stat";

lazy_static! {
    pub static ref PROC_STAT: String = if let Ok(proc) = std::env::var(DOCKER_PROC_DIR_ENV) {
        format!("{}/stat", proc)
    } else {
        String::from(PROC_STAT_DEFAULT)
    };
}

pub const USER: usize = 1;
pub const NICE: usize = 2;
pub const SYSTEM: usize = 3;
pub const IDLE: usize = 4;
pub const IOWAIT: usize = 5;
pub const IRQ: usize = 6;
pub const SOFTIRQ: usize = 7;
pub const STEAL: usize = 8;
pub const GUEST: usize = 9;
pub const GUEST_NICE: usize = 10;
