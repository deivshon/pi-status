use crate::status::DOCKER_PROC_DIR_ENV;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref PROC_PID_RE: Regex = Regex::new(r"/proc/[0-9]+$").unwrap();
    pub static ref PROC_DIR: String = if let Ok(proc) = std::env::var(DOCKER_PROC_DIR_ENV) {
        proc
    } else {
        String::from(PROC_DIR_DEFAULT)
    };
}

pub const PROC_DIR_DEFAULT: &str = "/proc";

pub const STATE_OFFSET: usize = 2;

pub const PID: usize = 0;
pub const NAME: usize = 1;

pub const THREADS: usize = 19 - STATE_OFFSET;
pub const USER_TIME: usize = 13 - STATE_OFFSET;
pub const SYSTEM_TIME: usize = 14 - STATE_OFFSET;
pub const START_TIME: usize = 21 - STATE_OFFSET;
pub const RSS: usize = 23 - STATE_OFFSET;

pub const POSSIBLE_STATES: [&str; 13] = [
    "R", "S", "D", "Z", "T", "t", "W", "X", "x", "K", "W", "P", "I",
];
