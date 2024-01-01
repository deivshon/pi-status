mod consts;
pub mod err;

use self::consts::{
    NAME, PID, POSSIBLE_STATES, PROC_DIR, PROC_PID_RE, RSS, START_TIME, SYSTEM_TIME, THREADS,
    USER_TIME,
};
use self::err::{ProcDataCreationErr, ProcDataRetrievalErr};

use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};

use nix::unistd;
use rayon::prelude::*;
use serde::Serialize;

use anyhow::{Error, Result};

#[derive(Serialize, Clone)]
pub struct Process {
    pid: u64,
    name: String,
    mem: u64,
    threads: u16,
    cpu_usage: u64,
    start_time: u64,
}

pub struct ProcessData {
    pub processes: Vec<Process>,
    old_processes_map: HashMap<(u64, u64), u64>,
    page_size: u64,
}

impl ProcessData {
    pub fn new() -> Result<Self> {
        let page_size: u64;
        match unistd::sysconf(unistd::SysconfVar::PAGE_SIZE) {
            Ok(o) => {
                if let Some(ps) = o {
                    page_size = ps as u64
                } else {
                    return Err(Error::new(ProcDataCreationErr::PageSizeEmpty));
                }
            }
            Err(e) => return Err(Error::new(ProcDataCreationErr::PageSizeErr(e))),
        }

        return Ok(ProcessData {
            processes: Vec::new(),
            old_processes_map: HashMap::new(),
            page_size,
        });
    }

    pub fn update(&mut self) -> Result<()> {
        let new_processes_arc = Arc::new(Mutex::new(HashMap::new()));

        let processes_arc: Arc<Mutex<Vec<Process>>> = Arc::new(Mutex::new(Vec::new()));
        let files = fs::read_dir((*PROC_DIR).as_str())?
            .into_iter()
            .filter_map(Result::ok)
            .collect::<Vec<fs::DirEntry>>();

        files.par_iter().for_each(|pid| {
            let pid_dir: String;

            match Self::validate_pid_dir(pid) {
                Ok(d) => {
                    if let Some(dir_str) = d.path().to_str() {
                        pid_dir = dir_str.to_string();
                    } else {
                        return;
                    }
                }
                Err(_) => return,
            }

            let Ok(proc_stat) = fs::read_to_string(format!("{}/stat", pid_dir)) else {
                return;
            };

            if let Some(p) = self.get_proc_data(&proc_stat, new_processes_arc.clone()) {
                {
                    let mut processes = processes_arc.lock().unwrap();
                    processes.push(p);
                }
            }
        });

        {
            let new_processes_map = new_processes_arc.lock().unwrap();
            let processes = processes_arc.lock().unwrap();
            self.old_processes_map = new_processes_map.clone();
            self.processes = processes.clone();
        }
        return Ok(());
    }

    fn get_proc_data(
        &self,
        stat_data: &String,
        new_procs_arc: Arc<Mutex<HashMap<(u64, u64), u64>>>,
    ) -> Option<Process> {
        let mut proc_data: Process = Process {
            pid: 0,
            name: String::new(),
            mem: 0,
            threads: 0,
            cpu_usage: 0,
            start_time: 0,
        };

        let split_stat = stat_data.split_whitespace().collect::<Vec<&str>>();

        if let Ok(pid) = split_stat[PID].parse::<u64>() {
            proc_data.pid = pid;
        } else {
            return None;
        }

        // Second field is not the name, can't go on with parsing
        if !split_stat[NAME].starts_with("(") {
            return None;
        }

        // Push into process name first (and possibly only) part of name
        proc_data.name.push_str(split_stat[NAME]);

        // State index will be used to correctly index the rest of the field,
        // since whitespaces in names mess with the successive indexes
        let mut state_index: usize = 2;
        if !split_stat[NAME].ends_with(")") {
            // Name has spaces, find end and set state index and name accordingly
            while split_stat[state_index].len() != 1
                && !POSSIBLE_STATES.contains(&split_stat[state_index])
            {
                proc_data.name.push_str(" ");
                proc_data.name.push_str(split_stat[state_index]);

                state_index += 1;
            }
        }
        proc_data.name.remove(0);
        proc_data.name.pop();

        let Ok(threads) = split_stat[THREADS + state_index].parse::<u16>() else {
            return None;
        };
        proc_data.threads = threads;

        let Ok(mem) = split_stat[RSS + state_index].parse::<u64>() else {
            return None;
        };
        proc_data.mem = mem * self.page_size;

        let Ok(start_time) = split_stat[START_TIME + state_index].parse::<u64>() else {
            return None;
        };
        proc_data.start_time = start_time;

        // Parse CPU usage
        if let (Ok(user), Ok(sys)) = (
            split_stat[USER_TIME + state_index].parse::<u64>(),
            split_stat[SYSTEM_TIME + state_index].parse::<u64>(),
        ) {
            let mut new_procs = new_procs_arc.lock().unwrap();

            if let Some(old) = self
                .old_processes_map
                .get(&(proc_data.pid, proc_data.start_time))
            {
                proc_data.cpu_usage = (user + sys) - old;
                new_procs.insert((proc_data.pid, proc_data.start_time), user + sys);
            } else {
                proc_data.cpu_usage = user + sys;
                new_procs.insert((proc_data.pid, proc_data.start_time), proc_data.cpu_usage);
            }
        } else {
            return None;
        }

        return Some(proc_data);
    }

    fn validate_pid_dir(dir: &fs::DirEntry) -> Result<&fs::DirEntry> {
        let dir_name: String;
        match dir.path().into_os_string().into_string() {
            Ok(d) => dir_name = d,
            Err(_) => return Err(Error::new(ProcDataRetrievalErr::NotPidDir)),
        }

        if !PROC_PID_RE.is_match(&dir_name) {
            return Err(Error::new(ProcDataRetrievalErr::NotPidDir));
        }

        return Ok(dir);
    }
}
