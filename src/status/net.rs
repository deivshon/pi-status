use std::time::UNIX_EPOCH;
use std::fs;

use crate::utils;
use serde::Serialize;
use std::error::Error;

use super::StatusFields;

const NET_DIR: &str = "/sys/class/net/";
const RX_DIR: &str = "statistics/tx_bytes";
const TX_DIR: &str = "statistics/rx_bytes";

#[derive(Serialize)]
pub struct NetStats {
    interface: String,
    total_uploaded: u64,
    total_downloaded: u64,

    download_speed: f64,
    upload_speed: f64,
    ts: f64
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
    let files = fs::read_dir(NET_DIR)?;

    for file in files {
        add_interface_dir(&mut ifas, file).unwrap_or(());
    }

    let mut max_ifa: Option<NetStats> = None;
    for interface in ifas {
        if let Some(ifa) = interface.path().to_str() {
            match get_net_stats(&String::from(ifa)) {
                Ok(ns) => {
                    if let Some(m) = &max_ifa {
                        if ns.total_uploaded + ns.total_downloaded > m.total_uploaded + m.total_downloaded {
                            max_ifa = Some(ns);
                        }
                    }
                    else {
                        max_ifa = Some(ns);
                        continue;
                    }
                },
                Err(_) => continue
            }
        }
    }

    match max_ifa {
        Some(ifa) => return Ok(ifa.interface),
        None => return Ok(String::from("/\\/\\"))
    }
}

fn get_net_stats(interface: &String) -> Result<NetStats, Box<dyn Error>> {
    return Ok(NetStats {
        total_uploaded: utils::u64_from_file(format!("{}/{}", interface, RX_DIR))?,
        total_downloaded: utils::u64_from_file(format!("{}/{}", interface, TX_DIR))?,
        ts: UNIX_EPOCH.elapsed().unwrap().as_millis() as f64,

        download_speed: 0.0,
        upload_speed: 0.0,

        interface: interface.to_owned()
    })
}

fn get_first_result() -> Result<StatusFields, Box<dyn Error>> {
    let max_ifa = get_max_interface()?;
    let max_ifa_stats = get_net_stats(&max_ifa)?;

    return Ok(StatusFields::NetStats(Some(max_ifa_stats)));
}

fn get_diff(current: &NetStats, old: &NetStats) -> NetStats {
    let elapsed = current.ts - old.ts;

    return NetStats {
        interface: old.interface.to_owned(),
        total_uploaded: current.total_uploaded,
        total_downloaded: current.total_downloaded,

        upload_speed: (((current.total_uploaded - old.total_uploaded) as f64 / elapsed) * 1024.0).round(),
        download_speed: (((current.total_downloaded - old.total_downloaded) as f64 / elapsed) * 1024.0).round(),

        ts: current.ts
    }
}

pub fn get(current_stats: &Option<NetStats>) -> StatusFields {
    if let Some(ns) = current_stats {
        match get_net_stats(&ns.interface) {
            Ok(current_stats) => {
                return StatusFields::NetStats(Some(get_diff(&current_stats, ns)))
            },
            Err(_) => ()
        }
    }
    
    match get_first_result() {
        Err(_) => return StatusFields::NetStats(None),
        Ok(res) => return res
    }
}
