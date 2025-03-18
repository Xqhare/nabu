use std::cell::Cell;
use std::collections::VecDeque;
use std::path::Path;
use std::usize;

use athena::tools::bitreader::bitreader;

use crate::Number;
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

fn deserialize_xff_number(content: &mut VecDeque<u8>, byte_pos: &Cell<usize>) -> Result<XffValue, NabuError> {
    let mut signed = false;
    let mut float = false;
    let mut num_store: Vec<u8> = Default::default();
    if content.front() == Some(&45) {
        signed = true;
        num_store.push(content.pop_front().expect("num_bytes.front() == Some()"));
    }
    while content.len() > 0 {
        if content.front() >= Some(&48) && content.front() <= Some(&57) {
            num_store.push(content.pop_front().expect("num_bytes.len() > 0"));
        } else if content.front() == Some(&44) || content.front() == Some(&46) {
            num_store.push(content.pop_front().expect("num_bytes.len() > 0"));
            if float {
                return Err(NabuError::InvalidNumber(
                    byte_pos.get(),
                    "Multiple decimal points".to_string(),
                ));
            } else {
                float = true;
            };
        } else {
            return Err(NabuError::InvalidNumber(
                byte_pos.get(),
                format!("Unexpected character: {}", content.front().unwrap()),
            ));
        }
    }


    let num_as_str = num_store.iter().map(|x| *x as char).collect::<String>();
    if signed {
        let check_isize = &num_as_str.parse::<isize>();
        if check_isize.is_ok() {
            Ok(XffValue::Number(Number::from(
                check_isize.as_ref().unwrap(),
            )))
        } else {
            Err(NabuError::InvalidNumber(byte_pos.get(), num_as_str))
        }
    } else if float {
        let check_float = &num_as_str.parse::<f64>();
        if check_float.is_ok() {
            Ok(XffValue::Number(Number::from(
                check_float.as_ref().unwrap(),
            )))
        } else {
            Err(NabuError::InvalidNumber(byte_pos.get(), num_as_str))
        }
    } else {
        let check_usize = &num_as_str.parse::<usize>();
        if check_usize.is_ok() {
            Ok(XffValue::Number(Number::from(
                check_usize.as_ref().unwrap(),
            )))
        } else {
            Err(NabuError::InvalidNumber(byte_pos.get(), num_as_str))
        }
    }
}

fn deserialize_xff_text(content: &mut VecDeque<u8>, byte_pos: &Cell<usize>) -> Result<XffValue, NabuError> {

    let mut str_out: String = Default::default();
    while content.front().is_some() {
        let current_char = content.pop_front().unwrap();
        if current_char >= 8 && current_char <= 13 {
            // cmd chars
            match current_char {
                8 => {
                    // Backspace
                    str_out.push('\x08')
                }
                9 => {
                    // Horizontal Tab
                    str_out.push('\t')
                }
                10 => {
                    // Line Feed
                    str_out.push('\n')
                }
                11 => {
                    // Vertical Tab
                    str_out.push('\x0b')
                }
                12 => {
                    // Form Feed
                    str_out.push('\x0c')
                }
                13 => {
                    // Carriage Return
                    str_out.push('\r')
                }
                _ => {
                    unreachable!()
                }
            }
        } else if current_char >= 32 && current_char <= 126
            || current_char == 128
            || current_char >= 130 && current_char <= 140
            || current_char == 142
            || current_char >= 145 && current_char <= 156
            || current_char >= 158
        {
            str_out.push(char::from_u32(current_char as u32).unwrap());
        } else {
            return Err(NabuError::InvalidASCIIString(
                current_char,
                byte_pos.get(),
                1,
            ));
        }
    }
    Ok(XffValue::from(str_out))
}
