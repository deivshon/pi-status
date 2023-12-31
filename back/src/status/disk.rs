use super::DOCKER_MOUNTS_FILE_ENV;

use std::fs;

use lazy_static::lazy_static;
use nix::sys::statvfs::statvfs;
use serde::Serialize;

const PROC_MOUNTS_DEFAULT: &str = "/proc/mounts";

lazy_static! {
    static ref PROC_MOUNTS: String = if let Ok(mounts) = std::env::var(DOCKER_MOUNTS_FILE_ENV) {
        mounts
    } else {
        String::from(PROC_MOUNTS_DEFAULT)
    };
}

const FILESYSTEM: usize = 0;
const MOUNTPOINT: usize = 1;

const EXCLUDED_MOUNTS: &[&str; 6] = &["/proc", "/sys", "/run", "/dev", "/tmp", "/var"];

#[derive(Serialize)]
pub struct Disk {
    filesystem: String,
    mountpoint: String,
    total: u64,
    available: u64,
}

fn get_disks() -> Result<Vec<Disk>, std::io::Error> {
    let mut disks = vec![];
    let proc_mounts = fs::read_to_string((*PROC_MOUNTS).as_str())?;

    for l in proc_mounts.lines() {
        let split_mount = l.split_whitespace().collect::<Vec<&str>>();
        if split_mount.len() < 2 {
            continue;
        }

        if EXCLUDED_MOUNTS
            .iter()
            .any(|path| split_mount[MOUNTPOINT].starts_with(path))
        {
            continue;
        }

        let Ok(disk_stats) = statvfs(split_mount[1]) else {
            continue;
        };

        disks.push(Disk {
            filesystem: String::from(split_mount[FILESYSTEM]),
            mountpoint: String::from(split_mount[MOUNTPOINT]),
            total: disk_stats.block_size() * disk_stats.blocks(),
            available: disk_stats.block_size() * disk_stats.blocks_available(),
        })
    }

    return Ok(disks);
}

pub fn get() -> Option<Vec<Disk>> {
    match get_disks() {
        Ok(disks) => Some(disks),
        Err(e) => {
            eprintln!("Error in disks component: {}", e);
            None
        }
    }
}
