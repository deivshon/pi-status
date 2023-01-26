use super::StatusFields;

use std::fs;
use std::io;
use std::error::Error;
use std::fmt;

use regex;

use serde::Serialize;

#[derive(Debug)]
struct NotPidDir;

impl Error for NotPidDir {}

impl fmt::Display for NotPidDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The passed directory is not a PID directory")
    }
}

const PROC_DIR: &str = "/proc/";

const NAME: usize = 1;
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
    let pid_regex = regex::Regex::new(r"/proc/[0-9]+$").unwrap();

    let dir_name: String;
    match valid_dir.path().into_os_string().into_string() {
        Ok(d) => dir_name = d,
        Err(_) => return Err(Box::new(NotPidDir))
    }

    if !pid_regex.is_match(&dir_name) {
        return Err(Box::new(NotPidDir));
    }

    return Ok(valid_dir)
}

fn get_proc_data(proc_captures: regex::Captures) -> Option<Process> {
    let mut res: Process = Process {
        pid: 0,
        name: String::from(""),
        mem: 0,
        threads: 0
    };

    if let Some(name) = proc_captures.get(NAME) {
        res.name = String::from(name.as_str());
    } else {return None}
    
    if let Some(pid) = proc_captures.get(PID) {
        if let Ok(pid_num) = pid.as_str().parse::<u64>() {
            res.pid = pid_num;
        } else {return None}
    } else {return None}
    
    if let Some(mem) = proc_captures.get(MEM) {
        if let Ok(mem_num) = mem.as_str().parse::<u64>() {
            res.mem = mem_num;
        } else {return None}
    } else {return None}
    
    if let Some(threads) = proc_captures.get(THREADS) {
        if let Ok(threads_num) = threads.as_str().parse::<u16>() {
            res.threads = threads_num;
        } else {return None}
    } else {return None}

    return Some(res)
}

fn get_procs() -> Result<Vec<Process>, Box<dyn Error>> {
    let mut procs: Vec<Process> = vec![];
    let files = fs::read_dir(PROC_DIR)?;

    let proc_regex = regex::Regex::new(r"(?m)Name:\t *(.*)\n(?:.|\n)*?^Pid:\t *(.*)\n(?:.|\n)*?^VmRSS:\t *(.*) (?:.|\n)*?^Threads:\t *(.*)").unwrap();
    for pid in files {
        let mut pid_dir: String = String::new();

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

        match proc_regex.captures(&proc_status) {
            Some(c) => {
                if let Some(p) = get_proc_data(c) {
                    procs.push(p);
                }
            },
            None => continue
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
