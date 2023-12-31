mod consts;

use std::fs;

use nix::sys::statvfs::statvfs;
use serde::Serialize;

use self::consts::{EXCLUDED_MOUNTS, FILESYSTEM, MOUNTPOINT, PROC_MOUNTS};

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
