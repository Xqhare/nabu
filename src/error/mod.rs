use core::fmt;
use std::usize;

use crate::xff::value::XffValue;

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
    InvalidNumber(usize, String),
    /// The wrapped byte is not a valid array separator, making the array invalid
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the invalid array was found
    /// * `byte` - The invalid byte
    InvalidArray(usize, u8),
    /// The wrapped byte is not a valid object separator, making the object invalid
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the invalid object was found
    /// * `byte` - The invalid byte
    InvalidObject(usize, u8),

    /// The wrapped value is not a valid string. The invalid string ends at the wrapped position.
    ///
    /// # Parameters
    /// * `pos` - The position in the file where the invalid key was found
    /// * `key` - The invalid key
    InvalidKey(usize, XffValue),

    /// Nabu only supports Values up to a size of 1 petabyte
    ///
    /// # Parameters
    /// * `len` - The length of the value
    InvalidXFFValueLength(usize),

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
    TruncatedXFF(usize),

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
            NabuError::InvalidNumber(i, n) => write!(f, "Invalid number: {} at byte position {}", n, i),
            NabuError::InvalidArray(a, i) => write!(f, "Invalid array structure byte: {} at byte position {}. Expected an array separator", a, i),
            NabuError::InvalidObject(o, i) => write!(f, "Invalid object structure byte: {} at byte position {}. Expected an object separator", o, i),
            NabuError::InvalidKey(p, v) => write!(f, "Invalid non string key: {} at byte position {}", v, p),
            NabuError::InvalidXFFValueLength(len) => write!(f, "Invalid XFF value length: {} (max: 8 bytes / 18.446.744.073.709.551.615)", len),

            // Xff general serde errors
            NabuError::InvalidASCIIString(b, i, v) => write!(f, "Invalid ASCII character (according to xff specification version: {}): {} at byte position {}", b, v, i),
            NabuError::InvalidXFFExtension(ext, err) => write!(f, "Invalid {} extension, {}", ext, err),
            NabuError::InvalidXFFByte(b, i, v) => write!(f, "Invalid XFF byte: {} for Xff Version {} at byte position {}", b, v, i),
            NabuError::InvalidXFFValueForVersion(value, ver) => write!(f, "Invalid XffValue for xff specification version {}: {:?}", ver, value),

            // Xff v0 serde errors
            NabuError::InvalidASCIICommandCharacter(b, i) => write!(f, "Invalid ASCII command character: {} at byte position {}", b, i),

            // Xff file errors
            NabuError::MissingEM(u) => write!(f, "Missing End of File marker EM, end of file, at expected byte position {}.", u),
            NabuError::EmpthyXFF => write!(f, "Empthy XFF"),
            NabuError::TruncatedXFF(u) => write!(f, "Truncated XFF at byte position {}", u),
            NabuError::UnknownXFFVersion(ver) => write!(f, "Unknown XFF version: {}", ver),
            NabuError::InvalidXFFVersion(val, ver) => write!(f, "Invalid XffValue for XFF version. Value {}; Version {}", val, ver),
        }
    }
}
