use std::fs;

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
