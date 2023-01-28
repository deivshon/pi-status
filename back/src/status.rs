pub mod temp;
pub mod net;
pub mod cpu;
pub mod ram;
pub mod proc;

use std::thread;
use std::sync::{RwLock, Barrier};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time;

use serde::Serialize;

use lazy_static::lazy_static;

pub static PROC_AND_CPU: AtomicBool = AtomicBool::new(false);

// Since the status data is going to live for the whole execution anyways,
// use static instead of Arcs
lazy_static! {
    pub static ref STATUS: RwLock<Status> = RwLock::new(Status {
        temp: None,
        net_stats: None,
        cpu_usage: None,
        ram: None,
        proc: None
    });
    pub static ref STATUS_STR: RwLock<String> = RwLock::new(String::new());
    pub static ref PROC_CPU_SYNC: Barrier = Barrier::new(2);
}

#[derive(Serialize)]
pub struct Status {
    pub temp: Option<f32>,
    pub net_stats: Option<net::NetStats>,
    pub cpu_usage: Option<Vec<cpu::CpuUsage>>,
    pub ram: Option<ram::Ram>,
    pub proc: Option<Vec<proc::Process>>
}

pub enum StatusFields {
    Temp(Option<f32>),
    NetStats(Option<net::NetStats>),
    CpuUsage(Option<Vec<cpu::CpuUsage>>),
    Ram(Option<ram::Ram>),
    Proc(Option<Vec<proc::Process>>)
}

pub fn continous_update(field: StatusFields, ms: u64) {
    loop {
        let data: StatusFields;
        match &field {
            StatusFields::Temp(_) => data = temp::get(),
            StatusFields::NetStats(_) => {
                let status_ref = STATUS.read().unwrap();
                data = net::get(&status_ref.net_stats);
            },
            StatusFields::CpuUsage(_) => data = cpu::get(),
            StatusFields::Ram(_) => data = ram::get(),
            StatusFields::Proc(_) => data = proc::get()
        }

        {
            let mut status_ref = STATUS.write().unwrap();
            match data {
                StatusFields::Temp(t) => status_ref.temp = t,
                StatusFields::NetStats(n) => status_ref.net_stats = n,
                StatusFields::CpuUsage(u) => status_ref.cpu_usage = u,
                StatusFields::Ram(r) => status_ref.ram = r,
                StatusFields::Proc(p) => status_ref.proc = p
            };
        }


        // Make sure CPU and processes data does not go out of sync
        match &field {
            StatusFields::CpuUsage(_) | StatusFields::Proc(_) => {
                if PROC_AND_CPU.load(Ordering::Relaxed) {
                    PROC_CPU_SYNC.wait();
                }
            },
            _ => ()
        }


        {
            let status_ref = STATUS.read().unwrap();
            let mut status_str_ref = STATUS_STR.write().unwrap();

            *status_str_ref = serde_json::to_string(&*status_ref).unwrap();
        }

        thread::sleep(time::Duration::from_millis(ms));
    }
}
