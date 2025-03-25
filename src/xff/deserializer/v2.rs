use std::{
    borrow::Borrow, cell::Cell, collections::{BTreeMap, VecDeque},
};

use athena::tools::{checksum::crc32::{crc32_with_table, generate_crc32_lookuptable, Crc32Table}, leb128::deserialize_leb128_unsigned};

use crate::{
    error::NabuError,
    xff::value::XffValue,
};

use super::{deserialize_xff_key_value, deserialize_xff_number, deserialize_xff_text};

pub fn deserialize_xff_v2(content: &mut VecDeque<u8>) -> Result<XffValue, NabuError> {
    // somewhat expensive function
    // doesn't save enough to really matter in this codebase, but it's the thought that counts!
    let table: Crc32Table = generate_crc32_lookuptable();
    // version is byte 0 and on byte long in v2;
    let byte_pos: Cell<usize> = Cell::new(1);

    // check file checksum
    if !check_file_checksum(content, &table) {
        return Err(NabuError::InvalidFileChecksum(byte_pos.get(), 2));
    }

    let out = deserialize_xff_v2_value(content, byte_pos.borrow(), &table)?;

    // popped last 5 elements in check_file_checksum but not accounted for them
    byte_pos.set(byte_pos.get() + 5);

    // EM check
    if content.len() > 0 {
        if content[0] == 25 && content.len() == 1 {
            Ok(out)
        } else {
            Err(NabuError::TruncatedXFF(byte_pos.get(), 2))
        }
    } else {
        Err(NabuError::TruncatedXFF(byte_pos.get(), 2))
    }
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
fn deserialize_xff_value_checksum(content: &mut VecDeque<u8>, byte_pos: &Cell<usize>) -> Result<u32, NabuError> {
    // pop CHK
    if let Some(23) = content.pop_front() {
        byte_pos.set(byte_pos.get() + 1);
        let mut checksum: [u8; 4] = Default::default();
        for i in 0..4 {
            checksum[i] = content.pop_front().ok_or(NabuError::TruncatedXFFValueChecksum(byte_pos.get(), 2))?;
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
        Ok(res as usize)
    } else {
        Err(NabuError::InvalidXFFValueLength(byte_pos.get(), 2))
    }
}

pub fn deserialize_xff_v2_value(content: &mut VecDeque<u8>, byte_pos: &Cell<usize>, table: &Crc32Table) -> Result<XffValue, NabuError> {
    match content.pop_front().ok_or(NabuError::TruncatedXFFValue(byte_pos.get(), 2))? {
        0 => {
            let _ = content.pop_front();
            byte_pos.set(byte_pos.get() + 1);
            //NUL
            return Ok(XffValue::Null);
        }
        1 => deserialize_xff_v2_text(content, byte_pos, table),
        2 => deserialize_xff_v2_number(content, byte_pos, table),
        3 => deserialize_xff_v2_array(content, byte_pos, table),
        4 => deserialize_xff_v2_object(content, byte_pos, table),
        5 => deserialize_xff_v2_data(content, byte_pos, table),
        16 => {
            let _ = content.pop_front();
            byte_pos.set(byte_pos.get() + 1);
            //TRU
            return Ok(XffValue::Boolean(true));
        }
        17 => {
            let _ = content.pop_front();
            byte_pos.set(byte_pos.get() + 1);
            //FAL
            return Ok(XffValue::Boolean(false));
        }
        _ => {
            //Error
            return Err(NabuError::InvalidXFFByte(content[0], byte_pos.get(), 1));
        }
    }
}

fn deserialize_xff_v2_text(content: &mut VecDeque<u8>, byte_pos: &Cell<usize>, table: &Crc32Table) -> Result<XffValue, NabuError> {
    let len = deserialize_xff_v2_value_length(content, byte_pos)?;
    let data = content.drain(0..len).collect::<Vec<u8>>();
    byte_pos.set(byte_pos.get() + len);
    let txt_checksum = deserialize_xff_value_checksum(content, byte_pos)?;
    // check EV
    if content[0] != 24 {
        return Err(NabuError::MissingEV(byte_pos.get()));
    } else {
        let _ = content.pop_front();
        byte_pos.set(byte_pos.get() + 1);
    }
    // EV -> -1 ; Checksum -> -4; CHK -> -1
    byte_pos.set(byte_pos.get() - 6);
    // Return
    if txt_checksum != crc32_with_table(&data, table) {
        Err(NabuError::InvalidXFFValueChecksum(byte_pos.get(), 2))
    } else {
        // -len
        byte_pos.set(byte_pos.get() - len);
        let out = deserialize_xff_text(&mut data.into(), byte_pos);
        // Hack for after deser execution to set byte_pos
        byte_pos.set(byte_pos.get() + len);
        byte_pos.set(byte_pos.get() + 6);
        out
    }
}

fn deserialize_xff_v2_number(content: &mut VecDeque<u8>, byte_pos: &Cell<usize>, table: &Crc32Table) -> Result<XffValue, NabuError> {
    let len = deserialize_xff_v2_value_length(content, byte_pos)?;
    let mut data = content.drain(0..len).collect::<VecDeque<u8>>();
    byte_pos.set(byte_pos.get() + len);
    let num_checksum = deserialize_xff_value_checksum(content, byte_pos)?;
    // check EV
    if content[0] != 24 {
        return Err(NabuError::MissingEV(byte_pos.get()));
    } else {
        let _ = content.pop_front();
        byte_pos.set(byte_pos.get() + 1);
    }
    // EV -> -1 ; Checksum -> -4; CHK -> -1
    byte_pos.set(byte_pos.get() - 6);
    // Return
    if num_checksum != crc32_with_table(data.make_contiguous(), table) {
        Err(NabuError::InvalidXFFValueChecksum(byte_pos.get(), 2))
    } else {
        byte_pos.set(byte_pos.get() - len);
        let out = deserialize_xff_number(&mut data, byte_pos);
        byte_pos.set(byte_pos.get() + len);
        byte_pos.set(byte_pos.get() + 6);
        out
    }
}

fn deserialize_xff_v2_array(content: &mut VecDeque<u8>, byte_pos: &Cell<usize>, table: &Crc32Table) -> Result<XffValue, NabuError> {
    //ARY

    let len = deserialize_xff_v2_value_length(content, byte_pos)?;

    let mut ary_bind: Vec<XffValue> = Default::default();

    if len != 0 {
        let mut array_data = content.drain(0..len).collect::<VecDeque<u8>>();
        let array_checksum = deserialize_xff_value_checksum(content, byte_pos)?;

        if crc32_with_table(array_data.make_contiguous(), table) != array_checksum {
            return Err(NabuError::InvalidXFFValueChecksum(byte_pos.get(), 2));
        }
        ary_bind.push(deserialize_xff_v2_value(&mut array_data, byte_pos, table)?);
    }

    while content[0] != 24 && content.front().is_some() {
        if content[0] == 30 {
            if content[1] == 24 {
                // closing ARY
                let _ = content.pop_front();
                let _ = content.pop_front();
                byte_pos.set(byte_pos.get() + 2);
                return Ok(XffValue::from(ary_bind));
            } else {
                let _ = content.pop_front();
                byte_pos.set(byte_pos.get() + 1);
                // another value
                ary_bind.push(deserialize_xff_v2_value(content, byte_pos, table)?);
            }
        } else {
            break;
        }
    }

    // no trailing RS
    if content[0] == 24 {
        // closing ARY
        let _ = content.pop_front();
        byte_pos.set(byte_pos.get() + 1);

        return Ok(XffValue::from(ary_bind));
    } else {
        return Err(NabuError::InvalidArray(byte_pos.get(), content[0], 2));
    }
}

fn deserialize_xff_v2_object(content: &mut VecDeque<u8>, byte_pos: &Cell<usize>, table: &Crc32Table) -> Result<XffValue, NabuError> {
    //OBJ

    let len = deserialize_xff_v2_value_length(content, byte_pos)?;

    let mut obj_bind: BTreeMap<String, XffValue> = Default::default();

    if len != 0 {
        let mut obj_data = content.drain(0..len).collect::<VecDeque<u8>>();
        let obj_checksum = deserialize_xff_value_checksum(content, byte_pos)?;

        if crc32_with_table(obj_data.make_contiguous(), table) != obj_checksum {
            return Err(NabuError::InvalidXFFValueChecksum(byte_pos.get(), 2));
        }
        while obj_data[0] != 24 && obj_data.front().is_some() {
            let (key, value) = deserialize_xff_key_value(&mut obj_data, byte_pos, 2)?;
            obj_bind.insert(key, value);
            if obj_data[0] == 30 {
                if obj_data[1] == 24 {
                    // closing OBJ
                    let _ = obj_data.pop_front();
                    let _ = obj_data.pop_front();
                    byte_pos.set(byte_pos.get() + 2);
                    return Ok(XffValue::from(obj_bind));
                } else {
                    let _ = obj_data.pop_front();
                    byte_pos.set(byte_pos.get() + 1);
                    // another key value pair
                    continue;
                }
            } else {
                break;
            }
        }
    }

    // no trailing RS
    if content[0] == 24 {
        // closing ARY
        let _ = content.pop_front();
        byte_pos.set(byte_pos.get() + 1);

        return Ok(XffValue::from(obj_bind));
    } else {
        return Err(NabuError::InvalidObject(byte_pos.get(), content[0], 2));
    }
}

fn deserialize_xff_v2_data(content: &mut VecDeque<u8>, byte_pos: &Cell<usize>, table: &Crc32Table) -> Result<XffValue, NabuError> {
    let len = deserialize_xff_v2_value_length(content, byte_pos)?;
    let mut data = content.drain(0..len).collect::<VecDeque<u8>>();
    byte_pos.set(byte_pos.get() + len);
    let num_checksum = deserialize_xff_value_checksum(content, byte_pos)?;
    // check EV
    if content[0] != 24 {
        return Err(NabuError::MissingEV(byte_pos.get()));
    } else {
        let _ = content.pop_front();
        byte_pos.set(byte_pos.get() + 1);
    }
    // EV -> -1 ; Checksum -> -4; CHK -> -1
    byte_pos.set(byte_pos.get() - 6);
    // Return
    if num_checksum != crc32_with_table(data.make_contiguous(), table) {
        Err(NabuError::InvalidXFFValueChecksum(byte_pos.get(), 2))
    } else {
        byte_pos.set(byte_pos.get() - len);
        let out = deserialize_xff_number(&mut data, byte_pos);
        byte_pos.set(byte_pos.get() + len);
        byte_pos.set(byte_pos.get() + 6);
        out
    }
}
