use core::fmt;
use std::usize;

use crate::XffValue;

#[derive(Debug)]
pub enum NabuError {
    // -----------------------------------------------
    //                   external errors
    // -----------------------------------------------
    /// Wrapper for any and all std::io::Errors
    IoError(std::io::Error),

    // -----------------------------------------------
    //                   Xff v0 errors
    // -----------------------------------------------
    /// The file is missing the End of Text marker at the wrapped position
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the missing ETX was found
    MissingETX(usize),
    /// The file is missing the Data Link Escape marker at the wrapped position
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the missing DLE was found
    MissingDLE(usize),
    /// The file is missing the Escape marker at the wrapped position
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the missing ESC was found
    MissingESC(usize),
    /// A missing command character was encountered
    MissingCommandCharacter,

    // -----------------------------------------------
    //                   Xff v1 errors
    // -----------------------------------------------
    /// The file is missing the Text marker at the wrapped position
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the missing TXT was found
    MissingTXT(usize),
    /// The file is missing the Number marker at the wrapped position
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the missing NUM was found
    MissingNUM(usize),
    /// The file is missing the Array marker at the wrapped position
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the missing ARY was found
    MissingARY(usize),
    /// The file is missing the Object marker at the wrapped position
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the missing OBJ was found
    MissingOBJ(usize),
    /// The file is missing the Data marker at the wrapped position
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the missing DAT was found
    MissingDAT(usize),
    /// The file is missing the End of Value marker at the wrapped position
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the missing EV was found
    MissingEV(usize),

    /// The wrapped byte is not a valid number
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the invalid number was found
    /// * `String` - The invalid number
    /// * `version` - The XFF version
    InvalidNumber(usize, String, u8),
    /// The wrapped byte is not a valid array separator, making the array invalid
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the invalid array was found
    /// * `byte` - The invalid byte
    /// * `version` - The XFF version
    InvalidArray(usize, u8, u8),
    /// The wrapped byte is not a valid object separator, making the object invalid
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the invalid object was found
    /// * `byte` - The invalid byte
    /// * `version` - The XFF version
    InvalidObject(usize, u8, u8),

    /// The wrapped value is not a valid string. The invalid string ends at the wrapped position.
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the invalid key was found
    /// * `key` - The invalid key
    /// * `version` - The XFF version
    InvalidKey(usize, XffValue, u8),

    /// # Parameters
    /// * `len` - The length of the value
    /// * `version` - The XFF version
    InvalidXFFValueLength(usize, u8),

    /// # Parameters
    /// * `len` - The length of the value
    /// * `pos` - The position in the file where the invalid value was found
    /// * `version` - The XFF version
    XFFValueLengthTooLong(usize, usize, u8),

    // -----------------------------------------------
    //             Xff general serde errors
    // -----------------------------------------------
    /// Invalid, wrapped, ASCII character encountered at the wrapped position
    /// Has to be a valid String character
    ///
    /// # Parameters
    /// * `char` - The invalid character
    /// * `pos` - The position in the file where the invalid character was found
    /// * `version` - The XFF version
    InvalidASCIIString(u8, usize, u8),
    /// Invalid, wrapped Extension encountered
    /// The file is valid XFF, but not the correct extension
    ///
    /// ONLY USED IN SOME V0 CODE - DEPRECATED
    ///
    /// # Parameters
    /// * `ext` - The invalid extension
    /// * `err` - A more descriptive error message
    InvalidXFFExtension(String, String),
    /// Invalid Byte according to the XFF specification of the file version
    ///
    /// # Parameters
    /// * `byte` - The invalid byte
    /// * `pos` - The position in the file where the invalid byte was found
    /// * `version` - The XFF version
    InvalidXFFByte(u8, usize, u8),
    /// Invalid XFF value for the current XFF version
    ///
    /// # Parameters
    /// * `value` - The invalid value
    /// * `version` - The XFF version
    InvalidXFFValueForVersion(XffValue, u8),

    // -----------------------------------------------
    //                Xff v0 serde errors
    // -----------------------------------------------
    /// Invalid, wrapped, ASCII command character encountered at the wrapped position
    ///
    /// # Parameters
    /// * `char` - The invalid character
    /// * `pos` - The position in the file where the invalid character was found
    InvalidASCIICommandCharacter(u8, usize),

    // -----------------------------------------------
    //                   Xff file errors
    // -----------------------------------------------
    /// The file is missing the End of File marker
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the missing EM was found
    MissingEM(usize),

    /// Completely empty file, missing both version and end of file bytes
    EmpthyXFF,

    /// Truncated file, missing end of file byte
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the missing EM was found
    /// * `version` - The XFF version
    TruncatedXFF(usize, u8),

    /// Unknown XFF version
    ///
    /// # Parameters
    /// * `version` - The unknown version
    UnknownXFFVersion(u8),

    /// Invalid XFF version, the value is not for the correct version
    ///
    /// # Parameters
    /// * `value` - The invalid value
    /// * `version` - The invalid version
    InvalidXFFVersion(XffValue, u8),

    /// Invalid XFF file checksum
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the invalid checksum was found
    /// * `version` - The XFF version
    InvalidFileChecksum(usize, u8),

    /// Truncated XFF value
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the truncated value was found
    /// * `version` - The XFF version
    TruncatedXFFValue(usize, u8),

    /// Truncated XFF value checksum
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the truncated checksum was found
    /// * `version` - The XFF version
    TruncatedXFFValueChecksum(usize, u8),

    /// Invalid XFF value checksum
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the invalid checksum was found
    /// * `version` - The XFF version
    InvalidXFFValueChecksum(usize, u8),

    /// String contains non-ASCII characters
    ///
    /// # Parameters
    /// * `value` - The invalid String
    /// * `version` - The XFF version
    StringContainsNonASCII(String, u8),

    /// Number contains invalid character
    ///
    /// # Parameters
    /// * `char` - The invalid character
    /// * `value` - The invalid number
    /// * `version` - The XFF version
    NumberContainsInvalidCharacter(u8, String, u8),

    // -----------------------------------------------
    //                   Xff v3 errors
    // -----------------------------------------------
    /// Invalid marker parity at the wrapped position
    ///
    /// # Parameters
    /// * `byte` - The invalid byte
    /// * `pos` - The position in the file where the invalid parity was found
    InvalidMarkerParity(u8, usize),

    /// Index checksum mismatch for a parent type
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the index checksum was found
    IndexChecksumMismatch(usize),

    /// Invalid table schema (mismatch between column count and row data)
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the invalid schema was found
    InvalidTableSchema(usize),

    /// Unsupported parent type encountered during parsing
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the unsupported type was found
    UnsupportedParentType(usize),

    /// Metadata contains non-flat values (nested parent types) which is forbidden in XFF v3
    InvalidMetadata(String),
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
            // external errors
            NabuError::IoError(err) => err.fmt(f),

            // Xff v0 errors
            NabuError::MissingETX(u) => write!(f, "Missing ETX at byte position {}", u),
            NabuError::MissingDLE(u) => write!(f, "Missing DLE at byte position {}", u),
            NabuError::MissingESC(u) => write!(f, "Missing ESC at byte position {}", u),
            NabuError::MissingCommandCharacter => write!(f, "Missing command character"),

            // Xff v1 errors
            NabuError::MissingTXT(u) => write!(f, "Missing TXT at byte position {}", u),
            NabuError::MissingNUM(u) => write!(f, "Missing NUM at byte position {}", u),
            NabuError::MissingARY(u) => write!(f, "Missing ARY at byte position {}", u),
            NabuError::MissingOBJ(u) => write!(f, "Missing OBJ at byte position {}", u),
            NabuError::MissingDAT(u) => write!(f, "Missing DAT at byte position {}", u),
            NabuError::MissingEV(u) => write!(f, "Missing EV at byte position {}", u),
            NabuError::InvalidNumber(i, n, v) => write!(
                f,
                "Invalid XFF version {} number: {} at byte position {}",
                v, n, i
            ),
            NabuError::InvalidArray(a, i, v) => write!(
                f,
                "Invalid XFF version {} array structure byte: {} at byte position {}. Expected an array separator",
                v, a, i
            ),
            NabuError::InvalidObject(o, i, v) => write!(
                f,
                "Invalid XFF version {} object structure byte: {} at byte position {}. Expected an object separator",
                v, o, i
            ),
            NabuError::InvalidKey(p, val, ver) => write!(
                f,
                "Invalid XFF version {} non string key: {} at byte position {}",
                ver, val, p
            ),
            NabuError::InvalidXFFValueLength(len, v) => {
                write!(f, "Invalid XFF version {} value length: {}", v, len)
            }

            // Xff general serde errors
            NabuError::InvalidASCIIString(b, i, v) => write!(
                f,
                "Invalid ASCII character for XFF version{}: {} at byte position {}",
                v, b, i
            ),
            NabuError::InvalidXFFExtension(ext, err) => {
                write!(f, "Invalid {} extension, {}", ext, err)
            }
            NabuError::InvalidXFFByte(b, i, v) => write!(
                f,
                "Invalid XFF byte: {} for Xff Version {} at byte position {}",
                b, v, i
            ),
            NabuError::InvalidXFFValueForVersion(value, ver) => write!(
                f,
                "Invalid XffValue for xff specification version {}: {:?}",
                ver, value
            ),

            // Xff v0 serde errors
            NabuError::InvalidASCIICommandCharacter(b, i) => write!(
                f,
                "Invalid ASCII command character: {} at byte position {}",
                b, i
            ),

            // Xff file errors
            NabuError::MissingEM(u) => write!(
                f,
                "Missing End of File marker EM, end of file, at expected byte position {}.",
                u
            ),
            NabuError::EmpthyXFF => write!(f, "Empthy XFF"),
            NabuError::TruncatedXFF(u, v) => {
                write!(f, "Truncated XFF version {} at byte position {}", v, u)
            }
            NabuError::UnknownXFFVersion(ver) => write!(f, "Unknown XFF version: {}", ver),
            NabuError::InvalidXFFVersion(val, ver) => write!(
                f,
                "Invalid XffValue for XFF version {}. Value {};",
                ver, val
            ),
            NabuError::TruncatedXFFValue(u, v) => write!(
                f,
                "Truncated XFF version {} value at byte position {}; ",
                v, u
            ),
            NabuError::TruncatedXFFValueChecksum(u, v) => write!(
                f,
                "Truncated XFF version {} value checksum at byte position {}",
                v, u
            ),

            // checksum errors
            NabuError::InvalidFileChecksum(c, v) => {
                write!(f, "Invalid XFF version {} file checksum: {}", v, c)
            }
            NabuError::InvalidXFFValueChecksum(u, v) => write!(
                f,
                "Invalid XFF version {} value checksum at byte position {}",
                v, u
            ),

            // other errors
            NabuError::StringContainsNonASCII(s, v) => write!(
                f,
                "Invalid XFF version {} string contains non-ASCII characters: {}",
                v, s
            ),
            NabuError::NumberContainsInvalidCharacter(c, n, v) => write!(
                f,
                "Invalid XFF version {} number contains invalid character: {} in number: {}",
                v, c, n
            ),
            NabuError::XFFValueLengthTooLong(l, u, v) => write!(
                f,
                "Invalid XFF version {} value length too long: {} at byte position {}",
                v, l, u
            ),
            // Xff v3 errors
            NabuError::InvalidMarkerParity(b, i) => write!(
                f,
                "Invalid XFF version 3 marker parity: {} at byte position {}",
                b, i
            ),
            NabuError::IndexChecksumMismatch(i) => write!(
                f,
                "XFF version 3 index checksum mismatch at byte position {}",
                i
            ),
            NabuError::InvalidTableSchema(i) => {
                write!(
                    f,
                    "XFF version 3 invalid table schema at byte position {}",
                    i
                )
            }
            NabuError::UnsupportedParentType(i) => write!(
                f,
                "XFF version 3 unsupported parent type at byte position {}",
                i
            ),
            NabuError::InvalidMetadata(s) => write!(f, "XFF version 3 invalid metadata: {}", s),
        }
    }
}
