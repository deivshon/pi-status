use std::fmt;

#[derive(Debug)]
pub enum NetDataCreationError {
    NoInterface,
}

impl std::error::Error for NetDataCreationError {}

impl fmt::Display for NetDataCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NetDataCreationError::NoInterface => {
                write!(f, "Could not get a suitable network interface to measure")
            }
        }
    }
}
