use std::fs;

use anyhow::Result;

const TEMP_PATH: &str = "/sys/class/thermal/thermal_zone0/temp";

fn get_temp() -> Result<f32> {
    let temp_str = fs::read_to_string(TEMP_PATH)?;
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
