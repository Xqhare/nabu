use std::path::Path;

use crate::{
    XffValue,
    error::{NabuError, Result},
};

pub mod v0;
use crate::xff::serializer::v0::serialize_xff_v0;
pub mod v1;
use crate::xff::serializer::v1::serialize_xff_v1;
pub mod v2;
use crate::xff::serializer::v2::serialize_xff_v2;

/// Takes in a Vec of XffValues and serializes it into a byte vector
///
/// Determines the XFF version to use and calls the appropriate serializer
///
/// Because of version 0, the data argument has to be a vector, even if only one XffElement is
/// permissable as with version 1. In this case just "wrap" it in a vector, only the first element
/// is used.
///
/// # Arguments
/// * `path` - The path to the file to write
/// * `data` - The Vec of XffValues to write
/// * `ver` - The XFF version to use
///
/// # Errors
/// Returns IO errors when issues with reading the file from disk occur
pub fn serialize_xff(data: Vec<XffValue>, version: u8) -> Result<Vec<u8>> {
    match version {
        0 => serialize_xff_v0(data),
        1 => {
            if data.len() != 1 {
                let pos = 1;
                return Err(NabuError::TruncatedXFF(pos, version));
            }
            serialize_xff_v1(data)
        }
        2 => {
            if data.len() != 1 {
                let pos = 1;
                return Err(NabuError::TruncatedXFF(pos, version));
            }
            serialize_xff_v2(data)
        }
        _ => Err(NabuError::UnknownXFFVersion(version)),
    }
}

/// Writes a vector of bytes to a file
///
/// # Arguments
/// * `path` - The path to the file to write
/// * `data` - The vector of bytes to write
///
/// # Errors
/// Returns IO errors should issues with writing the file to disk arise
pub fn write_bytes_to_file(path: &Path, data: Vec<u8>) -> Result<()> {
    std::fs::write(path, data)?;
    Ok(())
}
