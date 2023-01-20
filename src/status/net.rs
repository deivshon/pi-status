use std::sync::{Arc, RwLock};
use std::fs;

use crate::utils;
use serde::Serialize;

use super::StatusFields;

#[derive(Serialize)]
pub struct NetStats {
    interface: String,
    total_uploaded: u64,
    total_downloaded: u64,
    download_speed: u64,
    upload_speed: u64
}


fn get_max_interface() -> Result<String, String> {
    let mut ifas: Vec<fs::DirEntry>  = vec![];
    for file in fs::read_dir("/sys/class/net/").unwrap() {
        match file {
            Ok(f) => match f.metadata() {
                Ok(m) => if !m.is_file() {
                        ifas.push(f)
                    }
                Err(e) => return Err(e.to_string())
            },
            Err(e) => return Err(e.to_string())
        }
    }

    let mut max_ifa: Option<NetStats> = None;
    for interface in ifas {
        match interface.path().to_str() {
            Some(ifa) => match get_net_stats(String::from(ifa)) {
                    Some(ns) => match &max_ifa {
                            None => {max_ifa = Some(ns); continue;},
                            Some(m) =>
                                if ns.total_uploaded + ns.total_uploaded < m.total_uploaded + m.total_downloaded {
                                    max_ifa = Some(ns);
                                }
                        }
                    _ => continue,
                }
            _ => ()
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
