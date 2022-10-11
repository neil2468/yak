use std::fmt;

/// A list specifying possible errors.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Value is not a valid address
    InvalidAddress,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAddress => write!(f, "value is not a valid address"),
        }
    }
}
