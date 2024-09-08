use std::collections::VecDeque;
use std::path::Path;

use crate::{error::NabuError, xff::value::XffValue};

pub mod v0;
use crate::xff::deserializer::v0::deserialize_xff_v0;
pub mod v1;
use crate::xff::deserializer::v1::deserialize_xff_v1;

/// Reads the content of a XFF file and returns a Vec
///
/// Reads the first byte of the file to determine the version and then calls the appropriate deserializer for the version
///
/// Because of the way v0 is implemented, it always returns a vector, for v1 it only has one
/// element
///
/// # Arguments
/// * `path` - The path to the file to read
///
/// # Errors
/// Returns IO errors when issues with reading the file from disk occur
/// Also returns `NabuError::UnknownXFFVersion` when the version is higher than the current highest version of the XFF format
pub fn deserialize_xff(path: &Path) -> Result<XffValue, NabuError> {
    //takes about 200ms for 300mb
    let mut content: VecDeque<u8> = std::fs::read(path)?.into();
    if content.len() == 1 {
        return Err(NabuError::MissingEM(2));
    } else if content.len() == 0 {
        return Err(NabuError::EmpthyXFF);
    }
    // check for 2 bytes is done
    match content[0] {
        0 => deserialize_xff_v0(&mut content),
        1 => Ok(deserialize_xff_v1(&mut content)?),
        _ => Err(NabuError::UnknownXFFVersion(content[0])),
    }
}
