pub mod temp;
pub mod net;
pub mod cpu;
pub mod ram;
pub mod proc;
pub mod host;
pub mod disk;

use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::sync::RwLock;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::Serialize;

use lazy_static::lazy_static;

pub static STATUS_LAST: AtomicU64 = AtomicU64::new(0);

pub const DOCKER_PROC_DIR_ENV: &str = "PST_PROC_DIR";
pub const DOCKER_MOUNTS_FILE_ENV: &str = "PST_MOUNTS_FILE";
pub const DOCKER_HOST_FILE_ENV: &str = "PST_HOST_FILE";
pub const DOCKER_NET_DIR_ENV: &str = "PST_NET_DIR";
pub const DOCKER_THERMAL_DIR_ENV: &str = "PST_THERMAL_DIR";

// Since the status data is going to live for the whole execution anyways,
// use static instead of Arcs
lazy_static! {
    pub static ref STATUS: RwLock<Status> = RwLock::new(Status {
        host: None,
        temp: None,
        net_stats: None,
        cpu_usage: None,
        disk: None,
        ram: None,
        proc: None,
    });

    pub static ref STATUS_STR: RwLock<String> = RwLock::new(String::new());
}

#[derive(Serialize)]
pub struct Status {
    host: Option<host::Host>,
    temp: Option<f32>,
    net_stats: Option<net::NetStats>,
    cpu_usage: Option<Vec<cpu::CpuUsage>>,
    ram: Option<ram::Ram>,
    disk: Option<Vec<disk::Disk>>,
    proc: Option<Vec<proc::Process>>
}

pub fn continous_update() {
    let mut just_run;
    loop {
        {
            let mut status_ref = STATUS.write().unwrap();

            status_ref.host = host::get();
            status_ref.temp = temp::get();
            status_ref.net_stats = net::get(&status_ref.net_stats);
            status_ref.ram = ram::get();
            status_ref.disk = disk::get();

            status_ref.proc = proc::get();
            status_ref.cpu_usage = cpu::get();
        }
        
        {
            let status_ref = STATUS.read().unwrap();
            let mut status_str_ref = STATUS_STR.write().unwrap();
            
            *status_str_ref = serde_json::to_string(&*status_ref).unwrap();
        }        

        just_run = true;
        while SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - STATUS_LAST.load(Ordering::Relaxed) > 10 ||
              just_run
        {
            thread::sleep(Duration::from_millis(1000));
            just_run = false;
        }
    }
}
