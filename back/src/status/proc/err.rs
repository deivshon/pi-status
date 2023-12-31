use std::fmt;

#[derive(Debug)]
pub enum ProcErr {
    NotPidDir,
}

impl std::error::Error for ProcErr {}

impl fmt::Display for ProcErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProcErr::NotPidDir => write!(f, "The passed directory is not a PID directory"),
        }
    }
}
