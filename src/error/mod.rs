use core::fmt;

use crate::XffValue;

#[derive(Debug)]
/// Error type for Nabu XFF operations
pub enum NabuError {
    // -----------------------------------------------
    //                   external errors
    // -----------------------------------------------
    /// Wrapper for any and all `std::io::Errors`
    IoError(std::io::Error),

    // -----------------------------------------------
    //                   Xff v0 errors
    // -----------------------------------------------
    /// The file is missing the End of Text marker at the wrapped position
    MissingETX(usize),
    /// The file is missing the Data Link Escape marker at the wrapped position
    MissingDLE(usize),
    /// The file is missing the Escape marker at the wrapped position
    MissingESC(usize),
    /// A missing command character was encountered
    MissingCommandCharacter,

    // -----------------------------------------------
    //                   Xff v1 errors
    // -----------------------------------------------
    /// The file is missing the Text marker at the wrapped position
    MissingTXT(usize),
    /// The file is missing the Number marker at the wrapped position
    MissingNUM(usize),
    /// The file is missing the Array marker at the wrapped position
    MissingARY(usize),
    /// The file is missing the Object marker at the wrapped position
    MissingOBJ(usize),
    /// The file is missing the Data marker at the wrapped position
    MissingDAT(usize),
    /// The file is missing the End of Value marker at the wrapped position
    MissingEV(usize),

    /// The wrapped byte is not a valid number
    InvalidNumber(usize, String, u8),
    /// The wrapped byte is not a valid array separator, making the array invalid
    InvalidArray(usize, u8, u8),
    /// The wrapped byte is not a valid object separator, making the object invalid
    InvalidObject(usize, u8, u8),

    /// The wrapped value is not a valid string. The invalid string ends at the wrapped position.
    InvalidKey(usize, XffValue, u8),

    /// Invalid value length
    InvalidXFFValueLength(usize, u8),

    /// Value length too long
    XFFValueLengthTooLong(usize, usize, u8),

    // -----------------------------------------------
    //             Xff general serde errors
    // -----------------------------------------------
    /// Invalid, wrapped, ASCII character encountered at the wrapped position
    InvalidASCIIString(u8, usize, u8),
    /// Invalid, wrapped Extension encountered
    InvalidXFFExtension(String, String),
    /// Invalid Byte according to the XFF specification of the file version
    InvalidXFFByte(u8, usize, u8),
    /// Invalid XFF value for the current XFF version
    InvalidXFFValueForVersion(XffValue, u8),

    // -----------------------------------------------
    //                Xff v0 serde errors
    // -----------------------------------------------
    /// Invalid, wrapped, ASCII command character encountered at the wrapped position
    InvalidASCIICommandCharacter(u8, usize),

    // -----------------------------------------------
    //                   Xff file errors
    // -----------------------------------------------
    /// The file is missing the End of File marker
    MissingEM(usize),

    /// Completely empty file, missing both version and end of file bytes
    EmptyXFF,

    /// Truncated file, missing end of file byte
    TruncatedXFF(usize, u8),

    /// Unknown XFF version
    UnknownXFFVersion(u8),

    /// Invalid XFF version, the value is not for the correct version
    InvalidXFFVersion(XffValue, u8),

    /// Invalid XFF file checksum
    InvalidFileChecksum(usize, u8),

    /// Truncated XFF value
    TruncatedXFFValue(usize, u8),

    /// Truncated XFF value checksum
    TruncatedXFFValueChecksum(usize, u8),

    /// Invalid XFF value checksum
    InvalidXFFValueChecksum {
        /// The checksum that was expected
        expected: u32,
        /// The actual checksum that was calculated
        actual: u32,
        /// The byte position where the checksum was found
        pos: usize,
        /// The XFF version
        version: u8,
    },

    /// String contains non-ASCII characters
    StringContainsNonASCII(String, u8),

    /// Number contains invalid character
    NumberContainsInvalidCharacter(u8, String, u8),

    // -----------------------------------------------
    //                   Xff v3 errors
    // -----------------------------------------------
    /// Invalid marker parity at the wrapped position
    InvalidMarkerParity {
        /// The byte with invalid parity
        actual: u8,
        /// The position in the file
        pos: usize,
    },

    /// Index checksum mismatch for a parent type
    IndexChecksumMismatch {
        /// The checksum that was expected
        expected: u32,
        /// The actual checksum that was calculated
        actual: u32,
        /// The byte position where the checksum was found
        pos: usize,
    },

    /// Invalid table schema
    InvalidTableSchema {
        /// The row index where the mismatch occurred
        row_index: usize,
        /// How many columns were expected
        expected_cols: usize,
        /// How many columns were actually found
        actual_cols: usize,
        /// The byte position
        pos: usize,
    },

    /// Unsupported parent type encountered during parsing
    UnsupportedParentType(usize),

    /// Metadata contains non-flat values (nested parent types) which is forbidden in XFF v3
    InvalidMetadata(String),
}

/// Result type for Nabu XFF operations
pub type Result<T> = std::result::Result<T, NabuError>;

impl From<std::io::Error> for NabuError {
    fn from(err: std::io::Error) -> Self {
        NabuError::IoError(err)
    }
}

impl fmt::Display for NabuError {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // external errors
            NabuError::IoError(err) => err.fmt(f),

            // Xff v0 errors
            NabuError::MissingETX(u) => write!(f, "Missing ETX at byte position {u}"),
            NabuError::MissingDLE(u) => write!(f, "Missing DLE at byte position {u}"),
            NabuError::MissingESC(u) => write!(f, "Missing ESC at byte position {u}"),
            NabuError::MissingCommandCharacter => write!(f, "Missing command character"),

            // Xff v1 errors
            NabuError::MissingTXT(u) => write!(f, "Missing TXT at byte position {u}"),
            NabuError::MissingNUM(u) => write!(f, "Missing NUM at byte position {u}"),
            NabuError::MissingARY(u) => write!(f, "Missing ARY at byte position {u}"),
            NabuError::MissingOBJ(u) => write!(f, "Missing OBJ at byte position {u}"),
            NabuError::MissingDAT(u) => write!(f, "Missing DAT at byte position {u}"),
            NabuError::MissingEV(u) => write!(f, "Missing EV at byte position {u}"),
            NabuError::InvalidNumber(i, n, v) => write!(
                f,
                "Invalid XFF version {v} number: {n} at byte position {i}",
            ),
            NabuError::InvalidArray(a, i, v) => write!(
                f,
                "Invalid XFF version {v} array structure byte: {a} at byte position {i}. Expected an array separator",
            ),
            NabuError::InvalidObject(o, i, v) => write!(
                f,
                "Invalid XFF version {v} object structure byte: {o} at byte position {i}. Expected an object separator",
            ),
            NabuError::InvalidKey(p, val, ver) => write!(
                f,
                "Invalid XFF version {ver} non string key: {val} at byte position {p}",
            ),
            NabuError::InvalidXFFValueLength(len, v) => {
                write!(f, "Invalid XFF version {v} value length: {len}")
            }

            // Xff general serde errors
            NabuError::InvalidASCIIString(b, i, v) => write!(
                f,
                "Invalid ASCII character for XFF version{v}: {b} at byte position {i}",
            ),
            NabuError::InvalidXFFExtension(ext, err) => {
                write!(f, "Invalid {ext} extension, {err}")
            }
            NabuError::InvalidXFFByte(b, i, v) => write!(
                f,
                "Invalid XFF byte: {b} for Xff Version {v} at byte position {i}",
            ),
            NabuError::InvalidXFFValueForVersion(value, ver) => write!(
                f,
                "Invalid XffValue for xff specification version {ver}: {value:?}",
            ),

            // Xff v0 serde errors
            NabuError::InvalidASCIICommandCharacter(b, i) => write!(
                f,
                "Invalid ASCII command character: {b} at byte position {i}",
            ),

            // Xff file errors
            NabuError::MissingEM(u) => write!(
                f,
                "Missing End of File marker EM at expected byte position {u}.",
            ),
            NabuError::EmptyXFF => write!(f, "Empty XFF"),
            NabuError::TruncatedXFF(u, v) => {
                write!(f, "Truncated XFF version {v} at byte position {u}")
            }
            NabuError::UnknownXFFVersion(ver) => write!(f, "Unknown XFF version: {ver}"),
            NabuError::InvalidXFFVersion(val, ver) => write!(
                f,
                "Invalid XffValue for XFF version {ver}. Value {val};",
            ),
            NabuError::TruncatedXFFValue(u, v) => write!(
                f,
                "Truncated XFF version {v} value at byte position {u}; ",
            ),
            NabuError::TruncatedXFFValueChecksum(u, v) => write!(
                f,
                "Truncated XFF version {v} value checksum at byte position {u}",
            ),

            // checksum errors
            NabuError::InvalidFileChecksum(c, v) => {
                write!(f, "Invalid XFF version {v} file checksum: {c}")
            }
            NabuError::InvalidXFFValueChecksum { expected, actual, pos, version } => write!(
                f,
                "Invalid XFF version {version} value checksum at byte position {pos}. Expected: {expected:08X}, Actual: {actual:08X}",
            ),

            // other errors
            NabuError::StringContainsNonASCII(s, v) => write!(
                f,
                "Invalid XFF version {v} string contains non-ASCII characters: {s}",
            ),
            NabuError::NumberContainsInvalidCharacter(c, n, v) => write!(
                f,
                "Invalid XFF version {v} number contains invalid character: {c} in number: {n}",
            ),
            NabuError::XFFValueLengthTooLong(l, u, v) => write!(
                f,
                "Invalid XFF version {v} value length too long: {l} at byte position {u}",
            ),
            // Xff v3 errors
            NabuError::InvalidMarkerParity { actual, pos } => write!(
                f,
                "Invalid XFF version 3 marker parity: {actual:02X} at byte position {pos}",
            ),
            NabuError::IndexChecksumMismatch { expected, actual, pos } => write!(
                f,
                "XFF version 3 index checksum mismatch at byte position {pos}. Expected: {expected:08X}, Actual: {actual:08X}",
            ),
            NabuError::InvalidTableSchema { row_index, expected_cols, actual_cols, pos } => {
                write!(
                    f,
                    "XFF version 3 invalid table schema at byte position {pos}. Row {row_index} expected {expected_cols} columns but found {actual_cols}",
                )
            }
            NabuError::UnsupportedParentType(i) => write!(
                f,
                "XFF version 3 unsupported parent type at byte position {i}",
            ),
            NabuError::InvalidMetadata(s) => write!(f, "XFF version 3 invalid metadata: {s}"),
        }
    }
}
