use core::fmt;

#[derive(Debug)]
pub enum NabuError {
    /// Wrapper for any and all std::io::Errors
    IoError(std::io::Error),

    /// The file is missing the End of File marker
    MissingEM(usize),
    /// The file is missing the End of Text marker at the wrapped position
    MissingETX(usize),
    /// The file is missing the Data Link Escape marker at the wrapped position
    MissingDLE(usize),
    /// The file is missing the Escape marker at the wrapped position
    MissingESC(usize),
    /// A missing command character was encountered
    MissingCommandCharacter,

    /// Invalid, wrapped, ASCII character encountered at the wrapped position
    /// Has to be a valid String character
    InvalidASCIIString(u8, usize),
    /// Invalid, wrapped, ASCII command character encountered at the wrapped position
    InvalidASCIICommandCharacter(u8, usize),
    /// Invalid, wrapped, Extension encountered decoding error.
    /// This means that the file is a valid XFF file but not the correct extension
    InvalidXFFExtension(String, String),
    /// Invalid Byte according to the XFF specification of the file version
    InvalidXFFByte(u8, usize),

    /// Completely empty file, missing both version and end of file bytes
    EmpthyXFF,
    /// Truncated file, missing end of file byte
    TruncatedXFF(usize),
    
    /// Unknown XFF version
    UnknownXFFVersion(u8),
}

#[allow(dead_code)]
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

            NabuError::MissingEM(u) => write!(f, "Missing End of File marker EM, end of file, at expected byte position {}.", u),
            NabuError::MissingETX(u) => write!(f, "Missing ETX at byte position {}", u),
            NabuError::MissingDLE(u) => write!(f, "Missing DLE at byte position {}", u),
            NabuError::MissingESC(u) => write!(f, "Missing ESC at byte position {}", u),
            NabuError::MissingCommandCharacter => write!(f, "Missing command character"),

            NabuError::InvalidASCIIString(b, i) => write!(f, "Invalid ASCII character: {} at byte position {}", b, i),
            NabuError::InvalidASCIICommandCharacter(b, i) => write!(f, "Invalid ASCII command character: {} at byte position {}", b, i),
            NabuError::InvalidXFFExtension(ext, err) => write!(f, "Invalid {} extension, {}", ext, err),
            NabuError::InvalidXFFByte(b, i) => write!(f, "Invalid XFF byte: {} at byte position {}", b, i),

            NabuError::EmpthyXFF => write!(f, "Empthy XFF"),
            NabuError::TruncatedXFF(u) => write!(f, "Truncated XFF at byte position {}", u),

            NabuError::UnknownXFFVersion(ver) => write!(f, "Unknown XFF version: {}", ver),
        }
    }
}
