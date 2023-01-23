pub mod temp;
pub mod net;
pub mod cpu;

use std::sync::{Arc, RwLock};
use serde::Serialize;

use std::thread;
use std::time;

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

pub fn continous_update(status: Arc<RwLock<Status>>, status_str: Arc<RwLock<String>>, field: StatusFields, ms: u64) {
    loop {
        let data: StatusFields;
        match &field {
            StatusFields::Temp(_) => data = temp::get(),
            StatusFields::NetStats(_) => {
                let status_ref = status.read().unwrap();
                data = net::get(&status_ref.net_stats);
            },
            StatusFields::CpuUsage(_) => data = cpu::get()
        }

        {
            let mut status_ref = status.write().unwrap();
            match data {
                StatusFields::Temp(t) => status_ref.temp = t,
                StatusFields::NetStats(n) => status_ref.net_stats = n,
                StatusFields::CpuUsage(u) => status_ref.cpu_usage = u
            };
        }

        {
            let status_ref = status.read().unwrap();
            let mut status_str_ref = status_str.write().unwrap();

            *status_str_ref = serde_json::to_string(&*status_ref).unwrap();
        }

        thread::sleep(time::Duration::from_millis(ms));
    }
}
