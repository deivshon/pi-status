use std::fs;
use serde::Serialize;
use nix::sys::statvfs::statvfs;

use crate::status::StatusFields;

const PROC_MOUNTS: &str = "/proc/mounts";

const FILESYSTEM: usize = 0;
const MOUNTPOINT: usize = 1;

const EXCLUDED_MOUNTS: &[&str; 5] = &[
    "/proc",
    "/sys",
    "/run",
    "/dev",
    "/tmp"
];

#[derive(Serialize)]
pub struct Disk {
    filesystem: String,
    mountpoint: String,
    total: u64,
    available: u64
}

fn get_disks() -> Result<Vec<Disk>, std::io::Error> {
    let mut disks = vec![];
    let proc_mounts = fs::read_to_string(PROC_MOUNTS)?;

    for l in proc_mounts.lines() {
        let split_mount = l.split_whitespace().collect::<Vec<&str>>();
        if split_mount.len() < 2 {continue}

        if EXCLUDED_MOUNTS.iter().any(|path| split_mount[MOUNTPOINT].starts_with(path))
        {
            continue
        }

        let Ok(disk_stats) = statvfs(split_mount[1]) else {continue};

        disks.push(Disk {
            filesystem: String::from(split_mount[FILESYSTEM]),
            mountpoint: String::from(split_mount[MOUNTPOINT]),
            total: disk_stats.block_size() * disk_stats.blocks(),
            available: disk_stats.block_size() * disk_stats.blocks_available()
        })
    }

    return Ok(disks)
}

pub fn get() -> StatusFields {
    match get_disks() {
        Ok(disks) => StatusFields::Disk(Some(disks)),
        Err(e) => {
            eprintln!("Error in disks component: {}", e);
            StatusFields::Disk(None)
        }
    }
}