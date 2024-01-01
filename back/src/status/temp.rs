mod consts;

use std::fs;

use anyhow::Result;

use self::consts::TEMP_PATH;

pub struct TempData {
    pub degrees: f32,
}

impl TempData {
    pub fn get() -> Result<Self> {
        let temp_str = fs::read_to_string((*TEMP_PATH).as_str())?;
        let temp = temp_str.replace("\n", "").parse::<f32>()?;

        return Ok(Self {
            degrees: temp / 1e3,
        });
    }
}
