use std::{
    borrow::Borrow,
    cell::Cell,
    collections::{BTreeMap, VecDeque},
};

use athena::{
    checksum::{Crc32Table, crc32_with_table, generate_crc32_lookuptable},
    encoding_and_decoding::deserialize_leb128_unsigned,
};

use crate::{Data, XffValue, error::NabuError};

use super::{deserialize_xff_key_value, deserialize_xff_number, deserialize_xff_text};

/// Deserializes XFF version 2 content.
///
/// # Errors
/// Errors if the content is malformed or truncated according to v2 specification.
pub fn deserialize_xff_v2(content: &mut VecDeque<u8>) -> Result<XffValue, NabuError> {
    // somewhat expensive function
    // doesn't save enough to really matter in this codebase, but it's the thought that counts!
    let table: Crc32Table = generate_crc32_lookuptable();
    // version is byte 1 and one byte long in v2;
    let byte_pos: Cell<usize> = Cell::new(1);

    // check file checksum
    if !check_file_checksum(content, &table) {
        // byte_pos should point to last byte of checksum
        return Err(NabuError::InvalidFileChecksum(
            content.len().saturating_sub(1),
            2,
        ));
    }

    let out = deserialize_xff_v2_value(content, byte_pos.borrow(), &table)?;

    // popped last 5 elements in check_file_checksum but not accounted for them on purpouse
    byte_pos.set(byte_pos.get() + 5);

    // EM check
    if content.is_empty() {
        return Err(NabuError::TruncatedXFF(byte_pos.get(), 2));
    } else if content[0] == 25 && content.len() == 1 {
        return Ok(out);
    }
    Err(NabuError::TruncatedXFF(byte_pos.get(), 2))
}

fn check_file_checksum(content: &mut VecDeque<u8>, crc_table: &Crc32Table) -> bool {
    // min len includes smallest possible value (NUL / TRU / FAL)
    if content.len() < 7 {
        return false;
    }
    // pop EM check for it later
    let last = content.pop_back();
    debug_assert!(last.is_some());
    // pop checksum, 4 bytes
    let mut checksum: [u8; 4] = Default::default();
    // I am reading in the checksum backwards, so I reverse it here
    for i in (0..4).rev() {
        checksum[i] = content.pop_back().unwrap();
    }
    // pop CHK
    content.pop_back();

    let file_checksum: u32 = u32::from_le_bytes(checksum);
    debug_assert!(content.len() == content.as_slices().0.len());
    if file_checksum == crc32_with_table(content.as_slices().0, crc_table) {
        // push last back
        content.push_back(last.unwrap());
        true
    } else {
        false
    }
}

#[inline]
fn deserialize_xff_v2_value_checksum(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
) -> Result<u32, NabuError> {
    // pop CHK
    if let Some(23) = content.pop_front() {
        byte_pos.set(byte_pos.get() + 1);
        let mut checksum: [u8; 4] = Default::default();
        for b in &mut checksum {
            *b = content
                .pop_front()
                .ok_or(NabuError::TruncatedXFFValueChecksum(byte_pos.get(), 2))?;
            byte_pos.set(byte_pos.get() + 1);
        }
        let checksum = u32::from_le_bytes(checksum);
        Ok(checksum)
    } else {
        Err(NabuError::TruncatedXFFValueChecksum(byte_pos.get(), 2))
    }
}

#[inline]
fn deserialize_xff_v2_value_length(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
) -> Result<usize, NabuError> {
    if let Ok((res, len)) = deserialize_leb128_unsigned(content.make_contiguous()) {
        let _ = content.drain(0..len as usize);
        byte_pos.set(byte_pos.get() + len as usize);
        Ok(res)
    } else {
        Err(NabuError::InvalidXFFValueLength(byte_pos.get(), 2))
    }
}

/// Deserializes a single XFF v2 value.
///
/// # Errors
/// Errors if the value is malformed or truncated.
pub fn deserialize_xff_v2_value(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
    table: &Crc32Table,
) -> Result<XffValue, NabuError> {
    if content.is_empty() {
        return Err(NabuError::TruncatedXFFValue(byte_pos.get(), 2));
    }
    byte_pos.set(byte_pos.get() + 1);
    match content.pop_front().unwrap() {
        0 => Ok(XffValue::Null),
        1 => deserialize_xff_v2_text(content, byte_pos, table),
        2 => deserialize_xff_v2_number(content, byte_pos, table),
        3 => deserialize_xff_v2_array(content, byte_pos, table),
        4 => deserialize_xff_v2_object(content, byte_pos, table),
        5 => deserialize_xff_v2_data(content, byte_pos, table),
        16 => Ok(XffValue::Boolean(true)),
        17 => Ok(XffValue::Boolean(false)),
        _ => Err(NabuError::InvalidXFFByte(content[0], byte_pos.get(), 2)),
    }
}

fn deserialize_xff_v2_text(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
    table: &Crc32Table,
) -> Result<XffValue, NabuError> {
    let len = deserialize_xff_v2_value_length(content, byte_pos)?;
    let data = content.drain(0..len).collect::<Vec<u8>>();
    byte_pos.set(byte_pos.get() + len);
    let txt_checksum = deserialize_xff_v2_value_checksum(content, byte_pos)?;
    // check EV
    if content[0] != 24 {
        return Err(NabuError::MissingEV(byte_pos.get()));
    }
    let _ = content.pop_front();
    byte_pos.set(byte_pos.get() + 1);
    
    // Return
    let actual_crc = crc32_with_table(&data, table);
    if txt_checksum == actual_crc {
        // -len
        byte_pos.set(byte_pos.get().saturating_sub(len).saturating_sub(6));
        let out = deserialize_xff_text(&mut data.into(), byte_pos, 2);
        // Hack for after deser execution to set byte_pos after checksum + EV
        byte_pos.set(byte_pos.get() + 6);
        out
    } else {
        Err(NabuError::InvalidXFFValueChecksum {
            expected: txt_checksum,
            actual: actual_crc,
            pos: byte_pos.get().saturating_sub(1),
            version: 2,
        })
    }
}

fn deserialize_xff_v2_number(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
    table: &Crc32Table,
) -> Result<XffValue, NabuError> {
    let len = deserialize_xff_v2_value_length(content, byte_pos)?;
    let mut data = content.drain(0..len).collect::<VecDeque<u8>>();
    byte_pos.set(byte_pos.get() + len);
    let num_checksum = deserialize_xff_v2_value_checksum(content, byte_pos)?;
    // check EV
    if content[0] != 24 {
        return Err(NabuError::MissingEV(byte_pos.get()));
    }
    let _ = content.pop_front();
    byte_pos.set(byte_pos.get() + 1);
    
    // Return
    let actual_crc = crc32_with_table(data.make_contiguous(), table);
    if num_checksum == actual_crc {
        byte_pos.set(byte_pos.get().saturating_sub(len).saturating_sub(6));
        let out = deserialize_xff_number(&mut data, byte_pos, 2);
        byte_pos.set(byte_pos.get() + 6);
        out
    } else {
        Err(NabuError::InvalidXFFValueChecksum {
            expected: num_checksum,
            actual: actual_crc,
            pos: byte_pos.get().saturating_sub(1),
            version: 2,
        })
    }
}

fn deserialize_xff_v2_array(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
    table: &Crc32Table,
) -> Result<XffValue, NabuError> {
    //ARY

    let len = deserialize_xff_v2_value_length(content, byte_pos)?;

    let mut ary_bind: Vec<XffValue> = Vec::default();

    if len != 0 {
        let mut array_data = content.drain(0..len).collect::<VecDeque<u8>>();
        byte_pos.set(byte_pos.get() + len);
        let array_checksum = deserialize_xff_v2_value_checksum(content, byte_pos)?;

        let actual_crc = crc32_with_table(array_data.make_contiguous(), table);
        if actual_crc != array_checksum {
            return Err(NabuError::InvalidXFFValueChecksum {
                expected: array_checksum,
                actual: actual_crc,
                pos: byte_pos.get().saturating_sub(1),
                version: 2,
            });
        }
        // no EV check -> 5
        byte_pos.set(byte_pos.get().saturating_sub(len).saturating_sub(5));

        // Complete refactor from v1

        while array_data.front() != Some(&24) && array_data.front().is_some() {
            let val = deserialize_xff_v2_value(&mut array_data, byte_pos, table)?;
            ary_bind.push(val);
            // Now RS into EV, or EV for end -> ARY EV is still in content at index 0
            // OR RS into value
            if array_data.len() == 1 && array_data[0] == 30 {
                if content[0] == 24 {
                    // closing ARY
                    let _ = content.pop_front();
                    byte_pos.set(byte_pos.get() + 2);
                    return Ok(XffValue::from(ary_bind));
                }
                return Err(NabuError::MissingEV(byte_pos.get()));
            } else if array_data.is_empty() {
                if content[0] == 24 {
                    let _ = content.pop_front();
                    byte_pos.set(byte_pos.get() + 1);
                    return Ok(XffValue::from(ary_bind));
                }
                return Err(NabuError::MissingEV(byte_pos.get()));
            }
            if array_data[0] != 30 {
                return Err(NabuError::InvalidArray(byte_pos.get(), content[0], 2));
            }
            let _ = array_data.pop_front();
            byte_pos.set(byte_pos.get() + 1);
        }
        Err(NabuError::InvalidArray(byte_pos.get(), content[0], 2))
    } else {
        // remove checksum, check ev
        let _ = content.drain(0..5);
        byte_pos.set(byte_pos.get() + 5);
        if content[0] == 24 {
            let _ = content.pop_front();
            byte_pos.set(byte_pos.get() + 1);
            Ok(XffValue::from(ary_bind))
        } else {
            Err(NabuError::MissingEV(byte_pos.get()))
        }
    }
}

fn deserialize_xff_v2_object(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
    table: &Crc32Table,
) -> Result<XffValue, NabuError> {
    //OBJ

    let len = deserialize_xff_v2_value_length(content, byte_pos)?;

    let mut obj_bind: BTreeMap<String, XffValue> = BTreeMap::default();

    if len != 0 {
        let mut obj_data = content.drain(0..len).collect::<VecDeque<u8>>();
        byte_pos.set(byte_pos.get() + len);
        let obj_checksum = deserialize_xff_v2_value_checksum(content, byte_pos)?;

        let actual_crc = crc32_with_table(obj_data.make_contiguous(), table);
        if actual_crc != obj_checksum {
            return Err(NabuError::InvalidXFFValueChecksum {
                expected: obj_checksum,
                actual: actual_crc,
                pos: byte_pos.get().saturating_sub(1),
                version: 2,
            });
        }
        byte_pos.set(byte_pos.get().saturating_sub(len));

        while obj_data.front().is_some() && obj_data[0] != 24 {
            let (key, value) = deserialize_xff_key_value(&mut obj_data, byte_pos, 2)?;
            obj_bind.insert(key, value);
            if obj_data[0] == 30 {
                if content.len() > 1 && content[1] == 24 {
                    // closing OBJ
                    let _ = obj_data.pop_front();
                    let _ = content.pop_front();
                    byte_pos.set(byte_pos.get() + 2);
                    return Ok(XffValue::from(obj_bind));
                }
                let _ = obj_data.pop_front();
                byte_pos.set(byte_pos.get() + 1);
                // another key value pair
                continue;
            }
            break;
        }
    } else {
        // remove checksum, check ev
        let _ = content.drain(0..5);
        byte_pos.set(byte_pos.get() + 5);
        if content[0] != 24 {
            return Err(NabuError::MissingEV(byte_pos.get()));
        }
        let _ = content.pop_front();
        byte_pos.set(byte_pos.get() + 1);
        return Ok(XffValue::from(obj_bind));
    }

    // no trailing RS
    if !content.is_empty() && content[0] == 24 {
        // closing ARY
        let _ = content.pop_front();
        byte_pos.set(byte_pos.get() + 1);
        Ok(XffValue::from(obj_bind))
    } else {
        Err(NabuError::InvalidObject(byte_pos.get(), content[0], 2))
    }
}

fn deserialize_xff_v2_data(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
    table: &Crc32Table,
) -> Result<XffValue, NabuError> {
    let len = deserialize_xff_v2_value_length(content, byte_pos)?;
    let data = content.drain(0..len).collect::<Vec<u8>>();
    byte_pos.set(byte_pos.get() + len);
    let num_checksum = deserialize_xff_v2_value_checksum(content, byte_pos)?;
    // check EV
    if content[0] != 24 {
        return Err(NabuError::MissingEV(byte_pos.get()));
    }
    let _ = content.pop_front();
    byte_pos.set(byte_pos.get() + 1);
    
    // Return
    let actual_crc = crc32_with_table(&data, table);
    if num_checksum == actual_crc {
        let out = XffValue::Data(Data::from(data));
        Ok(out)
    } else {
        Err(NabuError::InvalidXFFValueChecksum {
            expected: num_checksum,
            actual: actual_crc,
            pos: byte_pos.get().saturating_sub(1),
            version: 2,
        })
    }
}
