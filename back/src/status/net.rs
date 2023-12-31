use super::DOCKER_NET_DIR_ENV;

use std::fs;
use std::sync::{Mutex, MutexGuard};
use std::time::UNIX_EPOCH;

use anyhow::Result;
use lazy_static::lazy_static;
use serde::Serialize;

const NET_DIR_DEFAULT: &str = "/sys/class/net/";

const RX_DIR: &str = "statistics/tx_bytes";
const TX_DIR: &str = "statistics/rx_bytes";

lazy_static! {
    static ref LAST_TIMESTAMP: Mutex<NetTimestamp> = Mutex::new(NetTimestamp {
        old: UNIX_EPOCH.elapsed().unwrap().as_millis(),
        new: 0
    });
    static ref NET_DIR: String = if let Ok(net_dir) = std::env::var(DOCKER_NET_DIR_ENV) {
        net_dir
    } else {
        String::from(NET_DIR_DEFAULT)
    };
}

pub struct NetTimestamp {
    old: u128,
    new: u128,
}

#[derive(Serialize)]
pub struct NetStats {
    interface: String,
    upload_total: u64,
    download_total: u64,
    upload_speed: f64,
    download_speed: f64,
}

fn u64_from_file(path: String) -> Result<u64> {
    let file_content = fs::read_to_string(path)?;
    let num = file_content.replace("\n", "").parse::<u64>()?;

    return Ok(num);
}

fn add_interface_dir(
    dst: &mut Vec<fs::DirEntry>,
    dir: Result<fs::DirEntry, std::io::Error>,
) -> Result<()> {
    let dir_entry = dir?;

    if !dir_entry.metadata()?.is_file() {
        dst.push(dir_entry)
    }

    return Ok(());
}

fn get_max_interface() -> Option<String> {
    let mut interfaces: Vec<fs::DirEntry> = vec![];
    let Ok(files) = fs::read_dir((*NET_DIR).as_str()) else {
        return None;
    };

    for file in files {
        add_interface_dir(&mut interfaces, file).unwrap_or(());
    }

    let mut max_ifa: Option<NetStats> = None;
    for ifa in interfaces {
        let ifa_path = ifa.path();

        let Some(ifa_name) = ifa_path.to_str() else {
            continue;
        };
        let Ok(ifa_stats) = get_net_stats(&String::from(ifa_name), None) else {
            continue;
        };

        if let Some(m) = &max_ifa {
            if ifa_stats.upload_total + ifa_stats.download_total > m.upload_total + m.download_total
            {
                max_ifa = Some(ifa_stats);
            }
        } else {
            max_ifa = Some(ifa_stats);
            continue;
        }
    }

    match max_ifa {
        Some(ifa) => return Some(ifa.interface),
        None => return None,
    }
}

fn get_net_stats(
    interface: &String,
    timestamps: Option<&mut MutexGuard<NetTimestamp>>,
) -> Result<NetStats> {
    if let Some(ts) = timestamps {
        (*ts).new = UNIX_EPOCH.elapsed().unwrap().as_millis();
    }

    return Ok(NetStats {
        upload_total: u64_from_file(format!("{}/{}", interface, RX_DIR))?,
        download_total: u64_from_file(format!("{}/{}", interface, TX_DIR))?,

        download_speed: 0.0,
        upload_speed: 0.0,

        interface: interface.to_owned(),
    });
}

fn get_first_result(timestamps: &mut MutexGuard<NetTimestamp>) -> Option<Result<NetStats>> {
    let Some(max_ifa) = get_max_interface() else {
        return None;
    };
    let max_ifa_stats;

    match get_net_stats(&max_ifa, Some(timestamps)) {
        Ok(stats) => max_ifa_stats = stats,
        Err(e) => return Some(Err(e)),
    }

    return Some(Ok(max_ifa_stats));
}

fn get_diff(
    current: &NetStats,
    old: &NetStats,
    timestamps: &mut MutexGuard<NetTimestamp>,
) -> NetStats {
    let old_ts = (*timestamps).old as f64;
    let elapsed = UNIX_EPOCH.elapsed().unwrap().as_millis() as f64 - old_ts;
    (*timestamps).old = (*timestamps).new;

    return NetStats {
        interface: old.interface.to_owned(),
        upload_total: current.upload_total,
        download_total: current.download_total,

        upload_speed: (((current.upload_total - old.upload_total) as f64 / elapsed) * 1024.0)
            .round(),
        download_speed: (((current.download_total - old.download_total) as f64 / elapsed) * 1024.0)
            .round(),
    };
}

pub fn get(current_stats: &Option<NetStats>) -> Option<NetStats> {
    let mut last_timestamp = LAST_TIMESTAMP.lock().unwrap();
    if let Some(old_stats) = current_stats {
        if let Ok(new_stats) = get_net_stats(&old_stats.interface, Some(&mut last_timestamp)) {
            return Some(get_diff(&new_stats, old_stats, &mut last_timestamp));
        }
    }

    let Some(first_result) = get_first_result(&mut last_timestamp) else {
        return None;
    };

    match first_result {
        Ok(res) => Some(res),
        Err(e) => {
            eprintln!("Error in Net component: {}", e);
            None
        }
    }
}
