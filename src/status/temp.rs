use std::fs;
use crate::{Status};
use std::sync::{Arc, RwLock};
use std::time;
use std::thread;

const TEMP_PATH: &str = "/sys/class/thermal/thermal_zone0/temp";

pub fn get() -> crate::StatusFields {
    let temp;

    match fs::read_to_string(TEMP_PATH) {
        Ok(t) => temp = t.replace("\n", "").parse::<f32>(),
        Err(e) => return crate::StatusFields::Temp(None)
    }
    
    match temp {
        Ok(t) => return crate::StatusFields::Temp(Some(t / 1e3)),
        Err(e) => crate::StatusFields::Temp(None)
    }
}
