use crate::status::{DOCKER_HOST_FILE_ENV, DOCKER_PROC_DIR_ENV};

use lazy_static::lazy_static;

pub const HOST_PATH_DEFAULT: &str = "/etc/hostname";
pub const UPTIME_PATH_DEFAULT: &str = "/proc/uptime";

lazy_static! {
    pub static ref HOST_PATH: String = if let Ok(etc_hostname) = std::env::var(DOCKER_HOST_FILE_ENV)
    {
        etc_hostname
    } else {
        String::from(HOST_PATH_DEFAULT)
    };
    pub static ref UPTIME_PATH: String = if let Ok(proc) = std::env::var(DOCKER_PROC_DIR_ENV) {
        format!("{}/uptime", proc)
    } else {
        String::from(UPTIME_PATH_DEFAULT)
    };
}
