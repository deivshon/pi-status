use anyhow::Error;
use std::fmt;

#[derive(Debug)]
pub enum NetDataUpdateError {
    NoInterfaces(Error),
}

impl std::error::Error for NetDataUpdateError {}

impl fmt::Display for NetDataUpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NetDataUpdateError::NoInterfaces(e) => {
                write!(f, "Could not get network interfaces: {}", e)
            }
        }
    }
}
