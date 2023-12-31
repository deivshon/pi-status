use crate::status::DOCKER_PROC_DIR_ENV;

use lazy_static::lazy_static;

pub const PROC_MEMINFO_DEFAULT: &str = "/proc/meminfo";

const TOTAL_LABEL: &str = "MemTotal:";
const FREE_LABEL: &str = "MemFree:";
const AVAILABLE_LABEL: &str = "MemAvailable:";
const CACHED_LABEL: &str = "Cached:";

pub const TOTAL: usize = 0;
pub const FREE: usize = 1;
pub const AVAILABLE: usize = 2;
pub const CACHED: usize = 3;
pub const EXPECTED_MEM_VALUES: usize = 4;

pub const LABELS: &'static [&str] = &[TOTAL_LABEL, FREE_LABEL, AVAILABLE_LABEL, CACHED_LABEL];

lazy_static! {
    pub static ref PROC_MEMINFO: String = if let Ok(proc) = std::env::var(DOCKER_PROC_DIR_ENV) {
        format!("{}/meminfo", proc)
    } else {
        String::from(PROC_MEMINFO_DEFAULT)
    };
}
