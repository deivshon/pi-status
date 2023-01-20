pub mod temp;
pub mod dummy;
pub mod net;

use std::sync::{Arc, RwLock};
use serde::Serialize;

use std::thread;
use std::time;

#[derive(Serialize)]
pub struct Status {
    pub temp: Option<f32>,
    pub dummy: Option<dummy::DummyStruct>,
    pub net_stats: Option<net::NetStats>
}

pub enum StatusFields {
    Temp(Option<f32>),
    Dummy(Option<dummy::DummyStruct>),
    NetStats(Option<net::NetStats>)
}

pub fn continous_update(status: Arc<RwLock<Status>>, status_str: Arc<RwLock<String>>, field: StatusFields, ms: u64) {
    loop {
        let data: StatusFields;
        match &field {
            StatusFields::Temp(_) => data = temp::get(),
            StatusFields::Dummy(_) => data = dummy::get(),
            StatusFields::NetStats(_) => {
                let status_ref = status.read().unwrap();
                data = net::get(&status_ref.net_stats);
            },
        }

        {
            let mut status_ref = status.write().unwrap();
            match data {
                StatusFields::Temp(t) => status_ref.temp = t,
                StatusFields::Dummy(v) => status_ref.dummy = v,
                StatusFields::NetStats(n) => status_ref.net_stats = n,
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
