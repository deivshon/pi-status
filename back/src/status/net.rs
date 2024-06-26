mod consts;
pub mod err;

use std::collections::HashMap;
use std::fs;
use std::time::UNIX_EPOCH;

use anyhow::{Error, Result};
use serde::Serialize;

use self::consts::{NET_DIR, RX_DIR, TX_DIR};
use self::err::NetDataUpdateError;

#[derive(Serialize, Clone, Debug)]
pub struct IfaStats {
    pub interface: NetworkInterface,
    pub upload_total: u64,
    pub download_total: u64,
    pub upload_speed: f64,
    pub download_speed: f64,
    pub timestamp: u128,
    pub has_updated: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct NetworkInterface {
    interface_path: String,
    interface_name: String,
}

pub struct NetData {
    pub stats: HashMap<String, IfaStats>,
    old_stats: HashMap<String, IfaStats>,
}

fn u64_from_file(path: String) -> Result<u64> {
    let file_content = fs::read_to_string(path)?;
    let num = file_content.replace("\n", "").parse::<u64>()?;

    return Ok(num);
}

impl NetData {
    pub fn new() -> Self {
        NetData {
            stats: HashMap::new(),
            old_stats: HashMap::new(),
        }
    }

    pub fn update(&mut self) -> Result<()> {
        let current_interfaces = match Self::get_interfaces() {
            Ok(i) => i,
            Err(e) => return Err(Error::new(NetDataUpdateError::NoInterfaces(e))),
        };

        let mut new_stats: HashMap<String, IfaStats> = HashMap::new();
        for interface in current_interfaces {
            match Self::get_ifa_stats(&interface) {
                Ok(s) => {
                    new_stats.insert(interface.interface_name, s);
                }
                Err(_) => continue,
            }
        }

        for (old_ifa, old_stats) in &self.old_stats {
            let new_ifa_stats = match new_stats.get(old_ifa) {
                Some(s) => s,
                None => {
                    new_stats.insert(
                        old_ifa.to_string(),
                        IfaStats {
                            download_speed: old_stats.download_speed,
                            download_total: old_stats.download_total,
                            has_updated: false,
                            interface: old_stats.interface.clone(),
                            timestamp: old_stats.timestamp,
                            upload_speed: old_stats.upload_speed,
                            upload_total: old_stats.upload_total,
                        },
                    );
                    continue;
                }
            };

            new_stats.insert(
                old_ifa.to_string(),
                Self::compute_speed(&new_ifa_stats, &old_stats),
            );
        }

        self.old_stats = self.stats.clone();
        self.stats = new_stats;

        Ok(())
    }

    fn get_ifa_stats(interface: &NetworkInterface) -> Result<IfaStats> {
        let timestamp = UNIX_EPOCH.elapsed().unwrap().as_millis();

        return Ok(IfaStats {
            upload_total: u64_from_file(format!("{}/{}", interface.interface_path, RX_DIR))?,
            download_total: u64_from_file(format!("{}/{}", interface.interface_path, TX_DIR))?,

            download_speed: 0.0,
            upload_speed: 0.0,

            interface: interface.clone(),
            timestamp,
            has_updated: true,
        });
    }

    fn compute_speed(current: &IfaStats, old: &IfaStats) -> IfaStats {
        let elapsed = (current.timestamp - old.timestamp) as f64;

        return IfaStats {
            interface: old.interface.to_owned(),
            upload_total: current.upload_total,
            download_total: current.download_total,

            upload_speed: (((current.upload_total - old.upload_total) as f64 / elapsed) * 1024.0)
                .round(),
            download_speed: (((current.download_total - old.download_total) as f64 / elapsed)
                * 1024.0)
                .round(),
            timestamp: current.timestamp,
            has_updated: current.has_updated,
        };
    }

    fn get_interfaces() -> Result<Vec<NetworkInterface>> {
        let mut interfaces_entries: Vec<fs::DirEntry> = Vec::new();
        let mut interfaces: Vec<NetworkInterface> = Vec::new();

        let files = fs::read_dir((*NET_DIR).as_str())?;
        for entry_res in files {
            let dir_entry: fs::DirEntry;
            match entry_res {
                Ok(d) => dir_entry = d,
                Err(_) => continue,
            };

            if !dir_entry.metadata()?.is_file() {
                interfaces_entries.push(dir_entry)
            }
        }

        for interface in interfaces_entries {
            let interface_path = interface.path();
            let Some(interface_name) = interface_path.file_name() else {
                continue;
            };
            let Some(interface_name) = interface_name.to_str() else {
                continue;
            };
            let Some(interface_path) = interface_path.to_str() else {
                continue;
            };

            let interface_path = String::from(interface_path);
            if interfaces
                .iter()
                .map(|x| x.interface_path.clone())
                .collect::<Vec<String>>()
                .contains(&interface_path)
            {
                continue;
            }

            interfaces.push(NetworkInterface {
                interface_path: interface_path,
                interface_name: String::from(interface_name),
            });
        }

        return Ok(interfaces);
    }
}
