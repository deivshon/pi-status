mod consts;

use nix::sys::statvfs::statvfs;
use serde::Serialize;
use std::fs;
use std::io;

use self::consts::{EXCLUDED_MOUNTS, FILESYSTEM, MOUNTPOINT, PROC_MOUNTS};

#[derive(Serialize)]
pub struct FsData {
    filesystem: String,
    mountpoint: String,
    total: u64,
    available: u64,
}

#[derive(Serialize)]
pub struct DiskData {
    pub filesystems: Vec<FsData>,
}

impl DiskData {
    pub fn get() -> Result<Self, io::Error> {
        let mut filesystems = Vec::new();
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

            filesystems.push(FsData {
                filesystem: String::from(split_mount[FILESYSTEM]),
                mountpoint: String::from(split_mount[MOUNTPOINT]),
                total: disk_stats.block_size() * disk_stats.blocks(),
                available: disk_stats.block_size() * disk_stats.blocks_available(),
            })
        }

        return Ok(DiskData { filesystems });
    }
}
