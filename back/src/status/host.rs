mod consts;
pub mod err;

use std::fs;

use anyhow::{Error, Result};
use serde::Serialize;

use self::consts::{HOST_PATH, UPTIME_PATH};
use self::err::HostErr;

#[derive(Serialize)]
pub struct HostData {
    hostname: String,
    uptime: u64,
}

impl HostData {
    pub fn get() -> Result<Self> {
        let hostname;
        let uptime;

        match HostData::get_hostname() {
            Ok(h) => hostname = h,
            Err(e) => return Err(e),
        }

        match HostData::get_uptime() {
            Ok(u) => uptime = u,
            Err(e) => return Err(e),
        }

        return Ok(HostData { hostname, uptime });
    }

    fn get_hostname() -> Result<String> {
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
}
