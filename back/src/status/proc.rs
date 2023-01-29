use super::StatusFields;

use std::fs;
use std::io;
use std::fmt;
use std::error::Error;
use std::sync::atomic::{AtomicU64, Ordering};

use serde::Serialize;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug)]
struct NotPidDir;

impl Error for NotPidDir {}

impl fmt::Display for NotPidDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The passed directory is not a PID directory")
    }
}

pub static PAGE_SIZE: AtomicU64 = AtomicU64::new(0);

lazy_static! {
    // Matches /proc/pid directories
    static ref PROC_PID_RE: Regex = Regex::new(r"/proc/[0-9]+$").unwrap();
}

const PROC_DIR: &str = "/proc/";

const STATE_OFFSET: usize = 2;

const PID: usize = 0;
const NAME: usize = 1;

const THREADS: usize = 19 - STATE_OFFSET;
const USER_TIME: usize = 13 - STATE_OFFSET;
const SYSTEM_TIME: usize = 14 - STATE_OFFSET;
const RSS: usize = 23 - STATE_OFFSET;

const POSSIBLE_STATES: [&str; 13] = ["R", "S", "D", "Z", "T", "t", "W", "X", "x", "K", "W", "P", "I"];

#[derive(Serialize)]
pub struct Process {
    pid: u64,
    name: String,
    mem: u64,
    threads: u16,
    cpu_usage: u64
}

fn validate_pid_dir(dir: io::Result<fs::DirEntry>) -> Result<fs::DirEntry, Box<dyn Error>> {
    let valid_dir = dir?;

    let dir_name: String;
    match valid_dir.path().into_os_string().into_string() {
        Ok(d) => dir_name = d,
        Err(_) => return Err(Box::new(NotPidDir))
    }

    if !PROC_PID_RE.is_match(&dir_name) {
        return Err(Box::new(NotPidDir));
    }

    return Ok(valid_dir)
}

fn get_proc_data(stat: String) -> Option<Process> {
    let mut res: Process = Process {
        pid: 0,
        name: String::new(),
        mem: 0,
        threads: 0,
        cpu_usage: 0
    };

    let split_stat = stat.split_whitespace().collect::<Vec<&str>>();

    // Parse pid
    if let Ok(pid) = split_stat[PID].parse::<u64>() {
        res.pid = pid;
    } else {return None}
    
    // Second field is not the name, can't go on with parsing
    if !split_stat[NAME].starts_with("(") {return None}
    
    // Push into process name first (and possibly only) part of name
    res.name.push_str(split_stat[NAME]);

    // State index will be used to correctly index the rest of the field,
    // since whitespaces in names mess with the successive indexes
    let mut state_index: usize = 2;
    if !split_stat[NAME].ends_with(")") {
        // Name has spaces, find end and set state index and name accordingly
        while split_stat[state_index].len() != 1 &&
              !POSSIBLE_STATES.contains(&split_stat[state_index])
        {
            res.name.push_str(" ");
            res.name.push_str(split_stat[state_index]);

            state_index += 1;
        }
    }
    res.name.remove(0);
    res.name.pop();
    
    // Parse threads
    if let Ok(threads) = split_stat[THREADS + state_index].parse::<u16>() {
        res.threads = threads;
    } else {return None}

    // Parse CPU usage
    if let (Ok(user), Ok(sys)) =
           (split_stat[USER_TIME + state_index].parse::<u64>(),
            split_stat[SYSTEM_TIME + state_index].parse::<u64>(),
           )
    {
        res.cpu_usage = user + sys;
    }
    else {return None}

    // Parse memory usage
    let Ok(mem) = split_stat[RSS + state_index].parse::<u64>() else {return None};
    res.mem = mem * PAGE_SIZE.load(Ordering::Relaxed);

    return Some(res)
}

fn get_procs() -> Result<Vec<Process>, Box<dyn Error>> {
    let mut procs: Vec<Process> = vec![];
    let files = fs::read_dir(PROC_DIR)?;

    for pid in files {
        let pid_dir: String;

        match validate_pid_dir(pid) {
            Ok(d) =>
            if let Some(dir_str) = d.path().to_str() {
                pid_dir = dir_str.to_string();
            } else {continue},
            Err(_) => continue
        }

        let Ok(proc_stat) = fs::read_to_string(format!("{}/stat", pid_dir)) else {continue};

        if let Some(p) = get_proc_data(proc_stat) {
            procs.push(p);
        }
    }

    return Ok(procs)
}

pub fn get() -> StatusFields {
    if let Ok(proc_data) = get_procs() {
        return StatusFields::Proc(Some(proc_data));
    };

    return StatusFields::Proc(None)
}
