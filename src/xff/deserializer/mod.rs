use std::collections::VecDeque;
use std::path::Path;
use std::usize;

use athena::tools::bitreader::bitreader;

use crate::{error::NabuError, xff::value::XffValue};

pub mod v0;
use crate::xff::deserializer::v0::deserialize_xff_v0;
pub mod v1;
use crate::xff::deserializer::v1::deserialize_xff_v1;
pub mod v2;
use crate::xff::deserializer::v2::deserialize_xff_v2;

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
    // check for empty is done
    let ver = deserialize_xff_version(&mut content);
    match ver {
        0 => deserialize_xff_v0(&mut content),
        1 => deserialize_xff_v1(&mut content),
        2 => deserialize_xff_v2(&mut content),
        _ => Err(NabuError::UnknownXFFVersion(ver as u8)),
    }
}

fn deserialize_xff_version(content: &mut VecDeque<u8>) -> usize {
    let mut acc: usize = 0;
    loop {
        let bits = bitreader(content.pop_front().unwrap());
        let bits_acc: u8 = bits.iter().sum();
        if bits_acc == 8 {
            acc += 7;
        } else {
            acc += bits_acc as usize;
            return acc;
        }
    }
}
