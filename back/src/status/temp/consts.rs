use crate::status::DOCKER_THERMAL_DIR_ENV;

use lazy_static::lazy_static;

pub const TEMP_PATH_DEFAULT: &str = "/sys/class/thermal/thermal_zone0/temp";

lazy_static! {
    pub static ref TEMP_PATH: String = if let Ok(thermal) = std::env::var(DOCKER_THERMAL_DIR_ENV) {
        format!("{}/thermal_zone0/temp", thermal)
    } else {
        String::from(TEMP_PATH_DEFAULT)
    };
}
