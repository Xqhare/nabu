use core::fmt;

pub enum NabuError {
    IoError(std::io::Error),
    UnknownXFFVersion(u8),
    InvalidXFF(String),
}

pub type Result<T> = std::result::Result<T, NabuError>;

impl From<std::io::Error> for NabuError {
    fn from(err: std::io::Error) -> Self {
        NabuError::IoError(err)
    }
}

impl fmt::Display for NabuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NabuError::IoError(err) => err.fmt(f),
            NabuError::UnknownXFFVersion(ver) => write!(f, "Unknown XFF version: {}", ver),
            NabuError::InvalidXFF(err) => write!(f, "Invalid XFF: {}", err),
        }
    }
}
