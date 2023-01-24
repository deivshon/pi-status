pub mod temp;
pub mod net;
pub mod cpu;

use std::thread;
use std::sync::RwLock;
use std::time;

use serde::Serialize;

use lazy_static::lazy_static;

// Since the status data is going to live for the whole execution anyways,
// use static instead of Arcs
lazy_static! {
    pub static ref STATUS: RwLock<Status> = RwLock::new(Status {
        temp: None,
        net_stats: None,
        cpu_usage: None
    });
    pub static ref STATUS_STR: RwLock<String> = RwLock::new(String::new());
}

#[derive(Serialize)]
pub struct Status {
    pub temp: Option<f32>,
    pub net_stats: Option<net::NetStats>,
    pub cpu_usage: Option<Vec<cpu::CpuUsage>>
}

pub enum StatusFields {
    Temp(Option<f32>),
    NetStats(Option<net::NetStats>),
    CpuUsage(Option<Vec<cpu::CpuUsage>>)
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
            StatusFields::CpuUsage(_) => data = cpu::get()
        }

        {
            let mut status_ref = STATUS.write().unwrap();
            match data {
                StatusFields::Temp(t) => status_ref.temp = t,
                StatusFields::NetStats(n) => status_ref.net_stats = n,
                StatusFields::CpuUsage(u) => status_ref.cpu_usage = u
            };
        }

        {
            let status_ref = STATUS.read().unwrap();
            let mut status_str_ref = STATUS_STR.write().unwrap();

            *status_str_ref = serde_json::to_string(&*status_ref).unwrap();
        }

        thread::sleep(time::Duration::from_millis(ms));
    }
}
