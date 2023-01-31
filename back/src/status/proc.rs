use std::fs;
use std::io;
use std::fmt;
use std::sync::MutexGuard;
use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap;
use std::sync::Mutex;

use serde::Serialize;
use regex::Regex;
use lazy_static::lazy_static;

use anyhow::{Result, Error};

#[derive(Debug)]
enum ProcErr {
    NotPidDir
}

impl std::error::Error for ProcErr {}

impl fmt::Display for ProcErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProcErr::NotPidDir => write!(f, "The passed directory is not a PID directory")
        }
    }
}

pub static PAGE_SIZE: AtomicU64 = AtomicU64::new(0);

lazy_static! {
    // Matches /proc/pid directories
    static ref PROC_PID_RE: Regex = Regex::new(r"/proc/[0-9]+$").unwrap();

    static ref CPU_PROC_OLD: Mutex<HashMap<u64, u64>> = Mutex::new(HashMap::new());
    static ref CPU_PROC_NEW: Mutex<HashMap<u64, u64>> = Mutex::new(HashMap::new());
}

// The Cantor pairing function is a function used to gain a unique number starting from 2 others in input
macro_rules! cantor {
    ($a:expr, $b:expr) => {
        (($a + $b) * ($a + $b + 1) / 2) + $b
    };
}

const PROC_DIR: &str = "/proc/";

const STATE_OFFSET: usize = 2;

const PID: usize = 0;
const NAME: usize = 1;

const THREADS: usize = 19 - STATE_OFFSET;
const USER_TIME: usize = 13 - STATE_OFFSET;
const SYSTEM_TIME: usize = 14 - STATE_OFFSET;
const START_TIME: usize = 21 - STATE_OFFSET;
const RSS: usize = 23 - STATE_OFFSET;

const POSSIBLE_STATES: [&str; 13] = ["R", "S", "D", "Z", "T", "t", "W", "X", "x", "K", "W", "P", "I"];

#[derive(Serialize, Clone)]
pub struct Process {
    uid: u64,
    pid: u64,
    name: String,
    mem: u64,
    threads: u16,
    cpu_usage: u64,
    start_time: u64
}

fn validate_pid_dir(dir: io::Result<fs::DirEntry>) -> Result<fs::DirEntry> {
    let valid_dir = dir?;

    let dir_name: String;
    match valid_dir.path().into_os_string().into_string() {
        Ok(d) => dir_name = d,
        Err(_) => return Err(Error::new(ProcErr::NotPidDir))
    }

    if !PROC_PID_RE.is_match(&dir_name) {
        return Err(Error::new(ProcErr::NotPidDir));
    }

    return Ok(valid_dir)
}

fn get_proc_data(
    stat: &String,
    old_procs: &MutexGuard<HashMap<u64, u64>>,
    new_procs: &mut MutexGuard<HashMap<u64, u64>>
) -> Option<Process> 
{
    let mut res: Process = Process {
        uid: 0,
        pid: 0,
        name: String::new(),
        mem: 0,
        threads: 0,
        cpu_usage: 0,
        start_time: 0
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
    let Ok(threads) = split_stat[THREADS + state_index].parse::<u16>() else {return None};
    res.threads = threads;

    // Parse memory usage
    let Ok(mem) = split_stat[RSS + state_index].parse::<u64>() else {return None};
    res.mem = mem * PAGE_SIZE.load(Ordering::Relaxed);
    
    let Ok(start_time) = split_stat[START_TIME + state_index].parse::<u64>() else {return None};
    res.start_time = start_time;
    
    // Gain robust unique id by combining PID and start time
    res.uid = cantor!(res.pid, res.start_time);
    
    // Parse CPU usage
    if let (Ok(user), Ok(sys)) =
           (split_stat[USER_TIME + state_index].parse::<u64>(),
            split_stat[SYSTEM_TIME + state_index].parse::<u64>(),
           )
    {
        if let Some(old) = old_procs.get(&res.uid) {
            res.cpu_usage = (user + sys) - old;
            new_procs.insert(res.uid, user + sys);
        } else {
            res.cpu_usage = user + sys;
            new_procs.insert(res.uid, res.cpu_usage);
        }
    }
    else {return None}

    return Some(res)
}

fn get_procs() -> Result<Vec<Process>> {
    let old_procs = CPU_PROC_OLD.lock().unwrap();
    let mut new_procs = CPU_PROC_NEW.lock().unwrap();
    new_procs.clear();

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

        if let Some(p) = get_proc_data(&proc_stat, &old_procs, &mut new_procs) {
            procs.push(p);
        }
    }

    return Ok(procs)
}

fn replace_old_map() {
    let mut proc_old = CPU_PROC_OLD.lock().unwrap();
    let proc_new = CPU_PROC_NEW.lock().unwrap();

    *proc_old = proc_new.clone();
}

pub fn get() -> Option<Vec<Process>> {
    match get_procs() {
        Ok(proc_data) => {
            replace_old_map();
            return Some(proc_data);
        }
        Err(e) => {
            eprintln!("Error in Proc component: {}", e);
            return None
        }
    };
}
