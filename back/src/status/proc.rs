use super::StatusFields;

use std::fs;
use std::io;
use std::error::Error;
use std::fmt;

use regex::Regex;
use lazy_static::lazy_static;

use serde::Serialize;

#[derive(Debug)]
struct NotPidDir;

impl Error for NotPidDir {}

impl fmt::Display for NotPidDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The passed directory is not a PID directory")
    }
}

lazy_static! {
    // Matches /proc/pid directories
    static ref PROC_PID_RE: Regex = Regex::new(r"/proc/[0-9]+$").unwrap();

    // Captures values of interest in the /proc/pid/status file
    static ref PROC_VALUE_RE: Regex = Regex::new(
        r"(?m)^Name:\t *(.*)|^Pid:\t *(.*)|^VmRSS:\t *(.*) |^Threads:\t *(.*)"
    ).unwrap();
}

const PROC_DIR: &str = "/proc/";

const NAME: usize =  1;
const PID: usize = 2;
const MEM: usize = 3;
const THREADS: usize = 4;

#[derive(Serialize)]
pub struct Process {
    pid: u64,
    name: String,
    mem: u64,
    threads: u16
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

fn get_proc_data(proc_captures: String) -> Option<Process> {
    let mut res: Process = Process {
        pid: 0,
        name: String::from(""),
        mem: 0,
        threads: 0
    };

    let captures: regex::CaptureMatches = PROC_VALUE_RE.captures_iter(&proc_captures);

    for c in captures {
        if let Some(n) = c.get(NAME) {
            res.name = n.as_str().to_string();
            continue;
        }

        if let Some(p) = c.get(PID) {
            if let Ok(pid) = p.as_str().parse::<u64>() {
                res.pid = pid;
            }
            continue;
        }

        if let Some(m) = c.get(MEM) {
            if let Ok(mem) = m.as_str().parse::<u64>() {
                res.mem = mem * 1024;
            }
            continue;
        }

        if let Some(t) = c.get(THREADS) {
            if let Ok(threads) = t.as_str().parse::<u16>() {
                res.threads = threads;
            }
            continue;
        }
    }

    if res.pid != 0 {
        return Some(res)
    } else {
        return None
    }
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
        if let Ok(content) = fs::read_to_string(format!("{}/status", pid_dir)) {
            proc_status = content;
        } else {continue}

        if let Some(p) = get_proc_data(proc_status) {
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
