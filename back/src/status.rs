pub mod temp;
pub mod net;
pub mod cpu;
pub mod ram;
pub mod proc;
pub mod host;
pub mod disk;

use std::thread;
use std::sync::RwLock;
use std::time;

use serde::Serialize;

use lazy_static::lazy_static;

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

pub fn continous_update(ms: u64) {
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

        thread::sleep(time::Duration::from_millis(ms));
    }
}
