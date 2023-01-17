use std::fs;
use crate::{Status};
use std::sync::{Arc, RwLock};
use std::time;
use std::thread;

const TEMP_PATH: &str = "/sys/class/thermal/thermal_zone0/temp";

pub fn get() -> Result<f32, String> {
    let temp;

    match fs::read_to_string(TEMP_PATH) {
        Ok(t) => temp = t.replace("\n", "").parse::<f32>(),
        Err(e) => return Err(e.to_string())
    }
    
    match temp {
        Ok(t) => return Ok(t / 1e3),
        Err(e) => return Err(e.to_string())
    }
}

pub fn continous_update(status: Arc<RwLock<Status>>, ms: u64) {
    loop {
        match get() {
            Ok(t) => {
                let mut d = status.write().unwrap();
                d.temp = t;
            },
            Err(_) => ()
        };

        thread::sleep(time::Duration::from_millis(ms));
    }
}
