mod consts;
pub mod err;

use std::fs;

use anyhow::{Error, Result};
use serde::Serialize;

use self::consts::{HOST_PATH, UPTIME_PATH};
use self::err::HostErr;

#[derive(Serialize)]
pub struct Host {
    hostname: String,
    uptime: u64,
}

fn get_hostname() -> Result<String, std::io::Error> {
    let hostname = fs::read_to_string((*HOST_PATH).as_str())?
        .trim_end()
        .to_string();

    return Ok(hostname);
}

fn get_uptime() -> Result<u64> {
    let uptime_unparsed = fs::read_to_string((*UPTIME_PATH).as_str())?;
    let uptime = uptime_unparsed.split(".").nth(0);

    match uptime {
        Some(u) => Ok(u.parse::<u64>()?),
        None => Err(Error::new(HostErr::MalformedUptimeFile)),
    }
}

pub fn get() -> Option<Host> {
    let hostname;
    let uptime;

    match get_hostname() {
        Ok(h) => hostname = h,
        Err(e) => {
            eprintln!("Error in Host component: Error retrieving hostname: {}", e);
            return None;
        }
    }

    match get_uptime() {
        Ok(u) => uptime = u,
        Err(e) => {
            eprintln!("Error in Host component: Error retrieving uptime: {}", e);
            return None;
        }
    }

    return Some(Host { hostname, uptime });
}
