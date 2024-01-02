pub mod cpu;
pub mod disk;
pub mod host;
pub mod net;
pub mod proc;
pub mod ram;
pub mod temp;

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use lazy_static::lazy_static;
use serde::Serialize;

use self::cpu::{CoreUsage, CpuUsage};
use self::disk::{DiskData, FsData};
use self::host::HostData;
use self::net::{IfaStats, NetData};
use self::proc::{Process, ProcessData};
use self::ram::RamData;
use self::temp::TempData;

pub static ACTIVE_WS_CONNECTIONS: AtomicU64 = AtomicU64::new(0);

pub const DOCKER_PROC_DIR_ENV: &str = "PST_PROC_DIR";
pub const DOCKER_MOUNTS_FILE_ENV: &str = "PST_MOUNTS_FILE";
pub const DOCKER_HOST_FILE_ENV: &str = "PST_HOST_FILE";
pub const DOCKER_NET_DIR_ENV: &str = "PST_NET_DIR";
pub const DOCKER_THERMAL_DIR_ENV: &str = "PST_THERMAL_DIR";

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
    pub static ref STATUS_STR: Arc<RwLock<String>> = Arc::new(RwLock::new(String::new()));
}

#[derive(Serialize)]
pub struct Status {
    host: Option<HostData>,
    temp: Option<f32>,
    net_stats: Option<IfaStats>,
    cpu_usage: Option<Vec<CoreUsage>>,
    ram: Option<RamData>,
    disk: Option<Vec<FsData>>,
    proc: Option<Vec<Process>>,
}

pub fn continous_update() {
    let mut just_run;
    let mut cpu_usage: CpuUsage = CpuUsage::new();
    let mut procs: Option<ProcessData> = match ProcessData::new() {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!("Could not start getting process data: {}. For this run process data will not be retrieved", e);
            None
        }
    };
    let mut net_data: Option<NetData> = match NetData::new() {
        Ok(n) => Some(n),
        Err(e) => {
            eprintln!("Could not start getting network data: {}. For this run network data will not be retrieved", e);
            None
        }
    };

    loop {
        {
            let mut status_ref = STATUS.write().unwrap();

            status_ref.host = match HostData::get() {
                Ok(h) => Some(h),
                Err(e) => {
                    eprintln!("Could not get host data: {}", e);
                    None
                }
            };
            status_ref.temp = match TempData::get() {
                Ok(t) => Some(t.degrees),
                Err(e) => {
                    eprintln!("Could not get temperature data: {}", e);
                    None
                }
            };

            status_ref.net_stats = match net_data {
                Some(ref mut n) => match n.update() {
                    Ok(()) => Some(n.stats.clone()),
                    Err(e) => {
                        eprintln!("Could not get network data: {}", e);
                        None
                    }
                },
                None => None,
            };

            status_ref.ram = match RamData::get() {
                Ok(r) => Some(r),
                Err(e) => {
                    eprintln!("Could not get RAM data: {}", e);
                    None
                }
            };
            status_ref.disk = match DiskData::get() {
                Ok(d) => Some(d.filesystems),
                Err(e) => {
                    eprintln!("Could not get disk data: {}", e);
                    None
                }
            };

            status_ref.proc = match procs {
                Some(ref mut p) => match p.update() {
                    Ok(()) => Some(p.processes.clone()),
                    Err(e) => {
                        eprintln!("Could not get processes data: {}", e);
                        None
                    }
                },
                None => None,
            };

            status_ref.cpu_usage = match cpu_usage.update() {
                Ok(()) => Some(cpu_usage.usage.clone()),
                Err(e) => {
                    eprintln!("Could not get CPU usage: {}", e);
                    None
                }
            }
        }

        {
            let status_ref = STATUS.read().unwrap();
            let mut status_str_ref = STATUS_STR.write().unwrap();

            *status_str_ref = serde_json::to_string(&*status_ref).unwrap();
        }

        just_run = true;
        while ACTIVE_WS_CONNECTIONS.load(Ordering::Relaxed) <= 0 || just_run {
            thread::sleep(Duration::from_millis(1000));
            just_run = false;
        }
    }
}
