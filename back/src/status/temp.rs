use super::DOCKER_THERMAL_DIR_ENV;

use std::fs;

use anyhow::Result;
use lazy_static::lazy_static;

const TEMP_PATH_DEFAULT: &str = "/sys/class/thermal/thermal_zone0/temp";

lazy_static! {
    static ref TEMP_PATH: String =
        if let Ok(thermal) = std::env::var(DOCKER_THERMAL_DIR_ENV) {
            format!("{}/thermal_zone0/temp", thermal)
        } else {
            String::from(TEMP_PATH_DEFAULT)
        };
}


fn get_temp() -> Result<f32> {
    let temp_str = fs::read_to_string((*TEMP_PATH).as_str())?;
    let temp = temp_str.replace("\n", "").parse::<f32>()?;

    return Ok(temp / 1e3);
}

pub fn get() -> Option<f32> {
    match get_temp() {
        Ok(t) => Some(t),
        Err(e) => {
            eprintln!("Error in Temp component: {}", e);
            None
        }
    }
}
