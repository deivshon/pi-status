use std::fs;

use crate::status::StatusFields;

const TEMP_PATH: &str = "/sys/class/thermal/thermal_zone0/temp";

pub fn get() -> StatusFields {
    let temp;

    match fs::read_to_string(TEMP_PATH) {
        Ok(t) => temp = t.replace("\n", "").parse::<f32>(),
        Err(_) => return StatusFields::Temp(None)
    }

    match temp {
        Ok(t) => return StatusFields::Temp(Some(t / 1e3)),
        Err(_) => StatusFields::Temp(None)
    }
}
