pub mod temp;
pub mod dummy;

use std::sync::{Arc, RwLock};
use serde::Serialize;

use std::thread;
use std::time;

#[derive(Serialize)]
pub struct Status {
    pub temp: Option<f32>,
    pub dummy: Option<dummy::DummyStruct>
}

pub enum StatusFields {
    Temp(Option<f32>),
    Dummy(Option<dummy::DummyStruct>)
}

pub fn continous_update(status: Arc<RwLock<Status>>, status_str: Arc<RwLock<String>>, field: StatusFields, ms: u64) {
    let mut get_func: fn() -> StatusFields;
    loop {
        match &field {
            StatusFields::Temp(_) => get_func = temp::get,
            StatusFields::Dummy(_) => get_func = dummy::get
        }


        let data = get_func();
        {
            let mut status_ref = status.write().unwrap();
            match data {
                StatusFields::Temp(t) => status_ref.temp = t,
                StatusFields::Dummy(v) => status_ref.dummy = v,
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
