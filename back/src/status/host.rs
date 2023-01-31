use std::fs;
use std::fmt;

use anyhow::{Result, Error};

use serde::Serialize;

use crate::status::StatusFields;

const HOST_PATH: &str = "/etc/hostname";
const UPTIME_PATH: &str = "/proc/uptime";

#[derive(Debug)]
struct MalformedUptimeFile;

impl std::error::Error for MalformedUptimeFile {}

impl fmt::Display for MalformedUptimeFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Uptime file is malformed and could not be parsed")
    }
}

#[derive(Serialize)]
pub struct Host {
    hostname: String,
    uptime: u64
}

fn get_hostname() -> Result<String, std::io::Error> {
    let hostname = fs::read_to_string(HOST_PATH)?.trim_end().to_string();

    return Ok(hostname)
}

fn get_uptime() -> Result<u64> {
    let uptime_unparsed = fs::read_to_string(UPTIME_PATH)?;
    let uptime = uptime_unparsed.split(".").nth(0);

    match uptime {
        Some(u) => Ok(u.parse::<u64>()?),
        None => Err(Error::new(MalformedUptimeFile))
    }
    
}

pub fn get() -> StatusFields {
    let hostname;
    let uptime;
    
    match get_hostname() {
        Ok(h) => hostname = h,
        Err(e) => {
            eprintln!("Error in Host component: Error retrieving hostname: {}", e);
            return StatusFields::Host(None)
        }
    }

    match get_uptime() {
        Ok(u) => uptime = u,
        Err(e) => {
            eprintln!("Error in Host component: Error retrieving uptime: {}", e);
            return StatusFields::Host(None)
        }
    }

    return StatusFields::Host(Some(Host {
        hostname,
        uptime
    }));
}
