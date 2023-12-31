use super::{DOCKER_HOST_FILE_ENV, DOCKER_PROC_DIR_ENV};

use std::fmt;
use std::fs;

use anyhow::{Error, Result};
use lazy_static::lazy_static;
use serde::Serialize;

const HOST_PATH_DEFAULT: &str = "/etc/hostname";
const UPTIME_PATH_DEFAULT: &str = "/proc/uptime";

lazy_static! {
    static ref HOST_PATH: String = if let Ok(etc_hostname) = std::env::var(DOCKER_HOST_FILE_ENV) {
        etc_hostname
    } else {
        String::from(HOST_PATH_DEFAULT)
    };
    static ref UPTIME_PATH: String = if let Ok(proc) = std::env::var(DOCKER_PROC_DIR_ENV) {
        format!("{}/uptime", proc)
    } else {
        String::from(UPTIME_PATH_DEFAULT)
    };
}

#[derive(Debug)]
enum HostErr {
    MalformedUptimeFile,
}

impl std::error::Error for HostErr {}

impl fmt::Display for HostErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HostErr::MalformedUptimeFile => {
                write!(f, "Uptime file is malformed and could not be parsed")
            }
        }
    }
}

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
