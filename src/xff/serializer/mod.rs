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
pub mod v3;
use crate::xff::serializer::v3::serialize_xff_v3;

/// Takes in a Vec of `XffValues` and serializes it into a byte vector
///
/// Determines the XFF version to use and calls the appropriate serializer
///
/// Because of version 0, the data argument has to be a vector, even if only one `XffElement` is
/// permissable as with version 1. In this case just "wrap" it in a vector, only the first element
/// is used.
///
/// # Arguments
/// * `data` - The Vec of `XffValues` to write
/// * `version` - The XFF version to use
///
/// # Errors
/// Returns `NabuError` when serialization fails or version is unknown.
pub fn serialize_xff(data: Vec<XffValue>, version: u8) -> Result<Vec<u8>> {
    match version {
        0 => serialize_xff_v0(data),
        1 => {
            if data.is_empty() {
                return Err(NabuError::TruncatedXFF(1, version));
            }
            serialize_xff_v1(&data)
        }
        2 => {
            if data.is_empty() {
                return Err(NabuError::TruncatedXFF(1, version));
            }
            serialize_xff_v2(&data)
        }
        3 => serialize_xff_v3(&data),
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
