use std::fmt;

#[derive(Debug)]
pub enum CpuErr {
    CoresChanged,
}

impl std::error::Error for CpuErr {}

impl fmt::Display for CpuErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CpuErr::CoresChanged => write!(f, "The number of cores changed"),
        }
    }
}
