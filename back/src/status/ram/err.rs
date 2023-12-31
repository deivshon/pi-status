use std::fmt;

#[derive(Debug)]
pub enum MemRetrievalErr {
    NotEnoughValues,
}

impl std::error::Error for MemRetrievalErr {}

impl fmt::Display for MemRetrievalErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MemRetrievalErr::NotEnoughValues => {
                write!(f, "Not enough values got during mem file parsing")
            }
        }
    }
}
