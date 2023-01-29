use super::StatusFields;

use std::fs;
use std::io;
use std::fmt;
use std::error::Error;
use std::sync::atomic::AtomicU64;

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

const THREADS: usize = 19 - STATE_OFFSET;
const USER_TIME: usize = 13 - STATE_OFFSET;
const SYSTEM_TIME: usize = 14 - STATE_OFFSET;

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

fn get_proc_data(status: String, stat: String) -> Option<Process> {
    let mut res: Process = Process {
        pid: 0,
        name: String::new(),
        mem: 0,
        threads: 0,
        cpu_usage: 0
    };

    // Parse name
    if let Some(name_line) = status.lines().nth(0) {
        res.name = name_line.split_whitespace().collect::<Vec<&str>>().drain(1..).collect::<Vec<&str>>().join(" ");
    }

    // Parse mem
    for l in status.lines() {
        if !l.starts_with("VmRSS:") {continue}

        if let Some(mem_str) = l.split_whitespace().nth(1) {
            if let Ok(mem) = mem_str.parse::<u64>() {
                res.mem = mem * 1024;
            } else {return None}
            break;
        }
    }

    let split_stat = stat.split_whitespace().collect::<Vec<&str>>();

    // Parse pid
    if let Ok(pid) = split_stat[PID].parse::<u64>() {
        res.pid = pid;
    } else {return None}
    
    // Second field is not the name, can't go on with parsing
    if !split_stat[1].starts_with("(") {return None}

    // State index will be used to correctly index the rest of the field,
    // since whitespaces in names mess with indexes
    let mut state_index: usize = 2;
    if !split_stat[1].ends_with(")") {
        // Name has spaces, find end and set state index accordingly
        while !split_stat[state_index].ends_with(")") {
            state_index += 1;
        }
        state_index += 1;
    }
    
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

        let proc_status: String;
        let proc_stat: String;
        if let (Ok(proc_status_content), Ok(proc_stat_content)) =
               (fs::read_to_string(format!("{}/status", pid_dir)),
                fs::read_to_string(format!("{}/stat", pid_dir)))
        {
            proc_status = proc_status_content;
            proc_stat = proc_stat_content;
        } else {continue}

        if let Some(p) = get_proc_data(proc_status, proc_stat) {
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
