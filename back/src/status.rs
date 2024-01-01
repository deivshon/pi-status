pub mod cpu;
pub mod disk;
pub mod host;
pub mod net;
pub mod proc;
pub mod ram;
pub mod temp;

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use lazy_static::lazy_static;
use serde::Serialize;

use self::cpu::{CoreUsage, CpuUsage};
use self::disk::{DiskData, FsData};
use self::host::HostData;
use self::proc::ProcessData;
use self::ram::RamData;
use self::temp::TempData;

pub static STATUS_LAST: AtomicU64 = AtomicU64::new(0);

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
    pub static ref STATUS_STR: RwLock<String> = RwLock::new(String::new());
}

#[derive(Serialize)]
pub struct Status {
    host: Option<HostData>,
    temp: Option<f32>,
    net_stats: Option<net::NetStats>,
    cpu_usage: Option<Vec<CoreUsage>>,
    ram: Option<RamData>,
    disk: Option<Vec<FsData>>,
    proc: Option<Vec<proc::Process>>,
}

pub fn continous_update() {
    let mut just_run;
    let mut cpu_usage: CpuUsage = CpuUsage::new();
    let mut procs: Option<ProcessData> = match ProcessData::new() {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!("Process data can't be retrieved: {}", e);
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
            status_ref.net_stats = net::get(&status_ref.net_stats);
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

            match procs {
                Some(ref mut p) => {
                    status_ref.proc = match p.update() {
                        Ok(()) => Some(p.processes.clone()),
                        Err(e) => {
                            eprintln!("Could not get processes data: {}", e);
                            None
                        }
                    };
                }
                None => (),
            }

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
        while SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - STATUS_LAST.load(Ordering::Relaxed)
            > 10
            || just_run
        {
            thread::sleep(Duration::from_millis(1000));
            just_run = false;
        }
    }
}
