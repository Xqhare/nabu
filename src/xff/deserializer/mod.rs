use std::cell::Cell;
use std::collections::VecDeque;
use std::path::Path;

use athena::byte_bit::byte_bit_decoder;
use athena::checksum::{Crc32Table, generate_crc32_lookuptable};
use v1::deserialize_xff_v1_value;
use v2::deserialize_xff_v2_value;

use crate::{Data, Number};
use crate::{XffValue, error::NabuError};

pub mod v0;
use crate::xff::deserializer::v0::deserialize_xff_v0;
pub mod v1;
use crate::xff::deserializer::v1::deserialize_xff_v1;
pub mod v2;
use crate::xff::deserializer::v2::deserialize_xff_v2;
pub mod v3;
use crate::xff::deserializer::v3::deserialize_xff_v3;
pub mod v4;
use crate::xff::deserializer::v4::deserialize_xff_v4;

/// Reads the content of a XFF file and returns a `XffValue`
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
    let content: Vec<u8> = std::fs::read(path)?;
    if content.is_empty() {
        return Err(NabuError::EmptyXFF);
    }

    // Check for v3 Magic Number
    if content.starts_with(&[0x58, 0x46, 0x46, 0x56]) {
        let mut cursor = 4;
        let (ver, len) =
            athena::encoding_and_decoding::deserialize_version_bit_chain(&content[cursor..])
                .map_err(|_| NabuError::UnknownXFFVersion(0))?;
        cursor += len as usize;

        if ver == 3 {
            return deserialize_xff_v3(&content, &mut cursor);
        } else if ver == 4 {
            return deserialize_xff_v4(&content, &mut cursor);
        }
        return Err(NabuError::UnknownXFFVersion(u8::try_from(ver).unwrap_or(0)));
    }

    // Legacy path (v0-v2)
    let mut deque: VecDeque<u8> = content.into();
    let ver = deserialize_xff_version(&mut deque);
    match ver {
        0 => deserialize_xff_v0(&mut deque),
        1 => deserialize_xff_v1(&mut deque),
        2 => deserialize_xff_v2(&mut deque),
        _ => Err(NabuError::UnknownXFFVersion(u8::try_from(ver).unwrap_or(0))),
    }
}

#[inline]
fn deserialize_xff_version(content: &mut VecDeque<u8>) -> usize {
    let mut acc: usize = 0;
    loop {
        let bits = byte_bit_decoder(content.pop_front().unwrap());
        let bits_acc: u8 = bits.iter().sum();
        if bits_acc == 8 {
            acc += 7;
        } else {
            acc += bits_acc as usize;
            return acc;
        }
    }
}

fn deserialize_xff_key_value(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
    ver: u8,
) -> Result<(String, XffValue), NabuError> {
    let table: Crc32Table = generate_crc32_lookuptable();
    // GS
    if content[0] != 29 {
        return Err(NabuError::InvalidObject(byte_pos.get(), content[0], ver));
    }
    let _ = content.pop_front();
    byte_pos.set(byte_pos.get() + 1);

    let mut key_bytes: VecDeque<u8> = VecDeque::default();
    if ver < 2 {
        while content.len() > 1 {
            if content[1] == 31 && content[0] == 24 {
                key_bytes.push_back(content.pop_front().unwrap());
                break;
            }
            key_bytes.push_back(content.pop_front().unwrap());
        }
    } else {
        while content.front().is_some() {
            if content[0] == 23 && content.len() > 6 && content[5] == 24 && content[6] == 31 {
                let rest_val = content.drain(0..6).collect::<Vec<u8>>();
                key_bytes.extend(rest_val);
                break;
            }
            key_bytes.push_back(content.pop_front().unwrap());
        }
    }
    let key_bind = {
        match ver {
            1 => deserialize_xff_v1_value(&mut key_bytes, byte_pos)?,
            2 => deserialize_xff_v2_value(&mut key_bytes, byte_pos, &table)?,
            _ => {
                return Err(NabuError::UnknownXFFVersion(ver));
            }
        }
    };
    if !key_bind.is_string() {
        return Err(NabuError::InvalidKey(byte_pos.get(), key_bind, ver));
    }

    // US
    if content[0] != 31 {
        return Err(NabuError::InvalidObject(byte_pos.get(), content[0], ver));
    }
    let _ = content.pop_front();
    byte_pos.set(byte_pos.get() + 1);

    let value = {
        match ver {
            1 => deserialize_xff_v1_value(content, byte_pos)?,
            2 => deserialize_xff_v2_value(content, byte_pos, &table)?,
            _ => {
                unreachable!("Invalid version: {}", ver)
            }
        }
    };
    // Trailing GS
    if !content.is_empty() && content[0] != 29 {
        return Err(NabuError::InvalidObject(byte_pos.get(), content[0], ver));
    }
    let _ = content.pop_front();
    byte_pos.set(byte_pos.get() + 1);

    Ok((key_bind.into_string().expect("Checked for String!"), value))
}

#[inline]
fn deserialize_xff_data(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
    len: usize,
) -> XffValue {
    //DAT
    let data = content.drain(0..len).collect::<Vec<u8>>();
    byte_pos.set(byte_pos.get() + len);
    XffValue::from(Data::from(data))
}

fn deserialize_xff_number(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
    ver: u8,
) -> Result<XffValue, NabuError> {
    let mut signed = false;
    let mut float = false;
    let mut num_store: Vec<u8> = Vec::default();
    if content.front() == Some(&45) {
        signed = true;
        num_store.push(content.pop_front().expect("num_bytes.front() == Some()"));
        byte_pos.set(byte_pos.get() + 1);
    }
    while !content.is_empty() {
        let front = *content.front().unwrap();
        if (48..=57).contains(&front) {
            num_store.push(content.pop_front().unwrap());
            byte_pos.set(byte_pos.get() + 1);
        } else if front == 44 || front == 46 {
            num_store.push(content.pop_front().unwrap());
            byte_pos.set(byte_pos.get() + 1);
            if float {
                return Err(NabuError::InvalidNumber(
                    byte_pos.get(),
                    "Multiple decimal points".to_string(),
                    2,
                ));
            }
            float = true;
        } else {
            return Err(NabuError::InvalidNumber(
                byte_pos.get(),
                format!("Unexpected character: {front}"),
                ver,
            ));
        }
    }

    let num_as_str = num_store.iter().map(|x| *x as char).collect::<String>();
    if signed && !float {
        if let Ok(val) = num_as_str.parse::<isize>() {
            Ok(XffValue::Number(Number::from(val)))
        } else {
            Err(NabuError::InvalidNumber(byte_pos.get(), num_as_str, 2))
        }
    } else if float {
        if let Ok(val) = num_as_str.parse::<f64>() {
            Ok(XffValue::Number(Number::from(val)))
        } else {
            Err(NabuError::InvalidNumber(byte_pos.get(), num_as_str, 2))
        }
    } else if let Ok(val) = num_as_str.parse::<usize>() {
        Ok(XffValue::Number(Number::from(val)))
    } else {
        Err(NabuError::InvalidNumber(byte_pos.get(), num_as_str, 2))
    }
}

fn deserialize_xff_text(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
    ver: u8,
) -> Result<XffValue, NabuError> {
    let mut str_out: String = String::default();
    while let Some(current_char) = content.pop_front() {
        byte_pos.set(byte_pos.get() + 1);
        if (8..=13).contains(&current_char) {
            // cmd chars
            match current_char {
                8 => {
                    // Backspace
                    str_out.push('\x08');
                }
                9 => {
                    // Horizontal Tab
                    str_out.push('\t');
                }
                10 => {
                    // Line Feed
                    str_out.push('\n');
                }
                11 => {
                    // Vertical Tab
                    str_out.push('\x0b');
                }
                12 => {
                    // Form Feed
                    str_out.push('\x0c');
                }
                13 => {
                    // Carriage Return
                    str_out.push('\r');
                }
                _ => {
                    unreachable!()
                }
            }
        } else if (32..=126).contains(&current_char)
            || current_char == 128
            || (130..=140).contains(&current_char)
            || current_char == 142
            || (145..=156).contains(&current_char)
            || current_char >= 158
        {
            str_out.push(char::from_u32(u32::from(current_char)).unwrap());
        } else {
            return Err(NabuError::InvalidASCIIString(
                current_char,
                byte_pos.get(),
                ver,
            ));
        }
    }
    Ok(XffValue::from(str_out))
}

#[test]
fn deser_version_0_10() {
    let mut ver0: VecDeque<u8> = vec![0].into();
    let mut ver1: VecDeque<u8> = vec![1].into();
    let mut ver2: VecDeque<u8> = vec![3].into();
    let mut ver3: VecDeque<u8> = vec![7].into();
    let mut ver4: VecDeque<u8> = vec![15].into();
    let mut ver5: VecDeque<u8> = vec![31].into();
    let mut ver6: VecDeque<u8> = vec![63].into();
    let mut ver7: VecDeque<u8> = vec![127].into();
    let mut ver8: VecDeque<u8> = vec![255, 1].into();
    let mut ver9: VecDeque<u8> = vec![255, 3].into();
    let mut ver10: VecDeque<u8> = vec![255, 7].into();

    assert_eq!(deserialize_xff_version(&mut ver0), 0);
    assert_eq!(deserialize_xff_version(&mut ver1), 1);
    assert_eq!(deserialize_xff_version(&mut ver2), 2);
    assert_eq!(deserialize_xff_version(&mut ver3), 3);
    assert_eq!(deserialize_xff_version(&mut ver4), 4);
    assert_eq!(deserialize_xff_version(&mut ver5), 5);
    assert_eq!(deserialize_xff_version(&mut ver6), 6);
    assert_eq!(deserialize_xff_version(&mut ver7), 7);
    assert_eq!(deserialize_xff_version(&mut ver8), 8);
    assert_eq!(deserialize_xff_version(&mut ver9), 9);
    assert_eq!(deserialize_xff_version(&mut ver10), 10);
}
