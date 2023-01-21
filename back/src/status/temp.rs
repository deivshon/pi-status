use std::fs;
use std::error::Error;

use crate::status::StatusFields;

const TEMP_PATH: &str = "/sys/class/thermal/thermal_zone0/temp";

fn get_temp() -> Result<StatusFields, Box<dyn Error>> {
    let temp_str = fs::read_to_string(TEMP_PATH)?;
    let temp = temp_str.replace("\n", "").parse::<f32>()?;

    return Ok(StatusFields::Temp(Some(temp / 1e3)));
}

pub fn get() -> StatusFields {
    let temp = get_temp();

    match temp {
        Ok(t) => t,
        Err(_) => StatusFields::Temp(None)
    }
}
