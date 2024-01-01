mod consts;
pub mod err;

use std::fs;
use std::time::UNIX_EPOCH;

use anyhow::{Error, Result};
use serde::Serialize;

use self::consts::{NET_DIR, RX_DIR, TX_DIR};
use self::err::NetDataCreationError;

#[derive(Serialize, Clone)]
pub struct IfaStats {
    interface: String,
    upload_total: u64,
    download_total: u64,
    upload_speed: f64,
    download_speed: f64,
    timestamp: u128,
}

pub struct NetData {
    pub stats: IfaStats,
    pub interface: String,
    old_stats: Option<IfaStats>,
}

fn u64_from_file(path: String) -> Result<u64> {
    let file_content = fs::read_to_string(path)?;
    let num = file_content.replace("\n", "").parse::<u64>()?;

    return Ok(num);
}

impl NetData {
    pub fn new() -> Result<Self> {
        let ifa = Self::get_max_interface();

        match ifa {
            Some(i) => Ok(Self {
                stats: IfaStats {
                    download_speed: 0.0,
                    download_total: 0,
                    interface: i.clone(),
                    upload_speed: 0.0,
                    upload_total: 0,
                    timestamp: 0,
                },
                interface: i,
                old_stats: None,
            }),
            None => Err(Error::new(NetDataCreationError::NoInterface)),
        }
    }

    pub fn update(&mut self) -> Result<()> {
        let current_stats = Self::get_ifa_stats(&self.interface)?;

        match &self.old_stats {
            Some(old) => {
                self.stats = Self::compute_speed(&current_stats, &old);
                self.old_stats = Some(self.stats.clone())
            }
            None => self.old_stats = Some(current_stats),
        }

        Ok(())
    }

    fn get_ifa_stats(interface: &String) -> Result<IfaStats> {
        let timestamp = UNIX_EPOCH.elapsed().unwrap().as_millis();

        return Ok(IfaStats {
            upload_total: u64_from_file(format!("{}/{}", interface, RX_DIR))?,
            download_total: u64_from_file(format!("{}/{}", interface, TX_DIR))?,

            download_speed: 0.0,
            upload_speed: 0.0,

            interface: interface.to_owned(),
            timestamp,
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
        };
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
        let mut interfaces: Vec<fs::DirEntry> = Vec::new();
        let Ok(files) = fs::read_dir((*NET_DIR).as_str()) else {
            return None;
        };

        for file in files {
            Self::add_interface_dir(&mut interfaces, file).unwrap_or(());
        }

        let mut max_ifa: Option<IfaStats> = None;
        for ifa in interfaces {
            let ifa_path = ifa.path();

            let Some(ifa_name) = ifa_path.to_str() else {
                continue;
            };
            let Ok(ifa_stats) = Self::get_ifa_stats(&String::from(ifa_name)) else {
                continue;
            };

            if let Some(m) = &max_ifa {
                if ifa_stats.upload_total + ifa_stats.download_total
                    > m.upload_total + m.download_total
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
}
