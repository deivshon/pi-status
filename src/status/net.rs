use std::sync::{Arc, RwLock};
use std::fs;

use crate::utils;
use serde::Serialize;
use std::error::Error;

use super::StatusFields;

#[derive(Serialize)]
pub struct NetStats {
    interface: String,
    total_uploaded: u64,
    total_downloaded: u64,
    download_speed: u64,
    upload_speed: u64
}

fn add_interface_dir(dst: &mut Vec<fs::DirEntry>, dir: Result<fs::DirEntry, std::io::Error>) -> Result<(), Box<dyn Error>>  {
    let dir_entry = dir?;

    if !dir_entry.metadata()?.is_file() {
        dst.push(dir_entry)
    }

    return Ok(())
}

fn get_max_interface() -> Result<String, Box<dyn Error>> {
    let mut ifas: Vec<fs::DirEntry>  = vec![];
    let files = fs::read_dir("/sys/class/net/")?;

    for file in files {
        add_interface_dir(&mut ifas, file).unwrap_or(());
    }

    let mut max_ifa: Option<NetStats> = None;
    for interface in ifas {
        if let Some(ifa) = interface.path().to_str() {
            if let Some(ns) = get_net_stats(String::from(ifa)) {
                if let Some(m) = &max_ifa {
                    if ns.total_uploaded + ns.total_uploaded < m.total_uploaded + m.total_downloaded {
                        max_ifa = Some(ns);
                    }
                }
                else {
                    max_ifa = Some(ns);
                    continue;
                }
            }
        }
    }

    return Ok(String::from("/"))
}

fn get_net_stats(interface: String) -> Option<NetStats> {
    return Some(NetStats {
        interface: String::from("/"),
        total_uploaded: 0,
        total_downloaded: 0,
        download_speed: 0,
        upload_speed: 0
    })
}

pub fn get(current_stats: &Option<NetStats>) -> StatusFields {
    get_max_interface().unwrap();
    if matches!(current_stats, None) {return StatusFields::NetStats(get_net_stats(String::from("/")))}
    
    return StatusFields::NetStats(None)
}
