use std::fmt;

use nix::errno::Errno;

#[derive(Debug)]
pub enum ProcDataRetrievalErr {
    NotPidDir,
}

#[derive(Debug)]
pub enum ProcDataCreationErr {
    PageSizeEmpty,
    PageSizeErr(Errno),
}

impl std::error::Error for ProcDataRetrievalErr {}
impl std::error::Error for ProcDataCreationErr {}

impl fmt::Display for ProcDataRetrievalErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProcDataRetrievalErr::NotPidDir => {
                write!(f, "The passed directory is not a PID directory")
            }
        }
    }
}

impl fmt::Display for ProcDataCreationErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PageSizeEmpty => write!(f, "Page size does not seem to exist"),
            Self::PageSizeErr(e) => write!(f, "Could not retrieve page size: {}", e),
        }
    }
}
