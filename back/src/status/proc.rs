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
        r"Name:\s*(.*)|[A-Za-z]*:\s*([0-9]*[^\sa-zA-Z])"
    ).unwrap();
}

const PROC_DIR: &str = "/proc/";

const NAME: &str = "Name";
const PID: &str = "Pid";
const MEM: &str = "VmRSS";
const THREADS: &str = "Threads";

const FIELDS: &[&str] = &[NAME, PID, MEM, THREADS];

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

    let mut capture: regex::Captures;
    for line in proc_captures.lines() {
        let line_split = line.split(":").collect::<Vec<&str>>();
        if line_split.len() < 2 {continue}
        if !FIELDS.contains(&line_split[0]) {continue}
        if let Some(c) = PROC_VALUE_RE.captures(line) {
            capture = c;
        } else {
            continue;
        }

        // Handle numeric values
        if let Some(n) = capture.get(2) {
            let value;
            if let Ok(v) = n.as_str().parse::<u64>() {
                value = v;
            } else {continue}

            match line_split[0] {
                PID => res.pid = value,
                MEM => res.mem = value * 1024,
                THREADS => res.threads = value as u16,
                _ => continue
            }
        }
        // Handle name
        else if let Some(n) = capture.get(1) {
            res.name = String::from(n.as_str());
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
