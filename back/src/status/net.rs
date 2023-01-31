use std::time::UNIX_EPOCH;
use std::fs;

use serde::Serialize;

use anyhow::Result;

const NET_DIR: &str = "/sys/class/net/";
const RX_DIR: &str = "statistics/tx_bytes";
const TX_DIR: &str = "statistics/rx_bytes";

#[derive(Serialize)]
pub struct NetStats {
    interface: String,
    upload_total: u64,
    download_total: u64,
    upload_speed: f64,
    download_speed: f64,
    ts: f64
}

fn u64_from_file(path: String) -> Result<u64> {
    let file_content = fs::read_to_string(path)?;
    let num = file_content.replace("\n", "").parse::<u64>()?;

    return Ok(num);
}

fn add_interface_dir(dst: &mut Vec<fs::DirEntry>, dir: Result<fs::DirEntry, std::io::Error>) -> Result<()>  {
    let dir_entry = dir?;

    if !dir_entry.metadata()?.is_file() {
        dst.push(dir_entry)
    }

    return Ok(())
}

fn get_max_interface() -> Option<String> {
    let mut interfaces: Vec<fs::DirEntry>  = vec![];
    let Ok(files) = fs::read_dir(NET_DIR) else {return None};

    for file in files {
        add_interface_dir(&mut interfaces, file).unwrap_or(());
    }

    let mut max_ifa: Option<NetStats> = None;
    for ifa in interfaces {
        let ifa_path = ifa.path();

        let Some(ifa_name) = ifa_path.to_str() else {continue};
        let Ok(ifa_stats) = get_net_stats(&String::from(ifa_name)) else {continue};

        if let Some(m) = &max_ifa {
            if ifa_stats.upload_total + ifa_stats.download_total > 
               m.upload_total + m.download_total
            {
                max_ifa = Some(ifa_stats);
            }
        }
        else {
            max_ifa = Some(ifa_stats);
            continue;
        }
    }

    match max_ifa {
        Some(ifa) => return Some(ifa.interface),
        None => return None
    }
}

fn get_net_stats(interface: &String) -> Result<NetStats> {
    return Ok(NetStats {
        upload_total: u64_from_file(format!("{}/{}", interface, RX_DIR))?,
        download_total: u64_from_file(format!("{}/{}", interface, TX_DIR))?,
        ts: UNIX_EPOCH.elapsed().unwrap().as_millis() as f64,

        download_speed: 0.0,
        upload_speed: 0.0,

        interface: interface.to_owned()
    })
}

fn get_first_result() -> Option<Result<NetStats>> {
    let Some(max_ifa) = get_max_interface() else {return None};
    let max_ifa_stats;

    match get_net_stats(&max_ifa) {
        Ok(stats) => max_ifa_stats = stats,
        Err(e) => return Some(Err(e))
    }

    return Some(Ok(max_ifa_stats));
}

fn get_diff(current: &NetStats, old: &NetStats) -> NetStats {
    let elapsed = current.ts - old.ts;

    return NetStats {
        interface: old.interface.to_owned(),
        upload_total: current.upload_total,
        download_total: current.download_total,

        upload_speed: (((current.upload_total - old.upload_total) as f64 / elapsed) * 1024.0).round(),
        download_speed: (((current.download_total - old.download_total) as f64 / elapsed) * 1024.0).round(),

        ts: current.ts
    }
}

pub fn get(current_stats: &Option<NetStats>) -> Option<NetStats> {
    if let Some(ns) = current_stats {
        match get_net_stats(&ns.interface) {
            Ok(current_stats) => {
                return Some(get_diff(&current_stats, ns))
            },
            Err(_) => ()
        }
    }

    let Some(first_result) = get_first_result() else {return None};

    match first_result {
        Ok(res) => Some(res),
        Err(e) => {
            eprintln!("Error in Net component: {}", e);
            None
        }
    }
}
