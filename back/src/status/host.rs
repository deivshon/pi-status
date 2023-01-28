use std::fs;
use std::error::Error;
use std::fmt;
use serde::Serialize;

use crate::status::StatusFields;

const HOST_PATH: &str = "/etc/hostname";
const UPTIME_PATH: &str = "/proc/uptime";

#[derive(Debug)]
struct MalformedUptimeFile;

impl Error for MalformedUptimeFile {}

impl fmt::Display for MalformedUptimeFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Uptime file is malformed and could not be parsed")
    }
}

#[derive(Serialize)]
pub struct Host {
    name: String,
    uptime: u64
}

fn get_hostname() -> Result<String, std::io::Error> {
    let hostname = fs::read_to_string(HOST_PATH)?.trim_end().to_string();

    return Ok(hostname)
}

fn get_uptime() -> Result<u64, Box<dyn Error>> {
    let uptime_unparsed = fs::read_to_string(UPTIME_PATH)?;
    let uptime = uptime_unparsed.split(".").nth(0);
    
    match uptime {
        Some(u) => Ok(u.parse::<u64>()?),
        None => Err(Box::new(MalformedUptimeFile))
    } 
    
}

pub fn get() -> StatusFields {
    let hostname;
    let uptime;
    if let (Ok(h), Ok(u)) = (get_hostname(), get_uptime()) {
        hostname = h;
        uptime = u;
    } else {return StatusFields::Host(None)}
    

    let res = Host {
        name: hostname,
        uptime: uptime
    };

    return StatusFields::Host(Some(res));
}
