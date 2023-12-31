use std::fmt;

#[derive(Debug)]
pub enum HostErr {
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
