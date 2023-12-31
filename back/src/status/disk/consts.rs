use crate::status::DOCKER_MOUNTS_FILE_ENV;

use lazy_static::lazy_static;

pub const PROC_MOUNTS_DEFAULT: &str = "/proc/mounts";

lazy_static! {
    pub static ref PROC_MOUNTS: String = if let Ok(mounts) = std::env::var(DOCKER_MOUNTS_FILE_ENV) {
        mounts
    } else {
        String::from(PROC_MOUNTS_DEFAULT)
    };
}

pub const FILESYSTEM: usize = 0;
pub const MOUNTPOINT: usize = 1;

pub const EXCLUDED_MOUNTS: &[&str; 6] = &["/proc", "/sys", "/run", "/dev", "/tmp", "/var"];
