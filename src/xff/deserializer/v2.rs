use std::{
    borrow::Borrow, cell::Cell, collections::{BTreeMap, VecDeque},
};

use athena::tools::{bitreader::bitreader, checksum::crc32::{crc32_with_table, generate_crc32_lookuptable, Crc32Table}, leb128::deserialize_leb128_unsigned};

use crate::{
    error::NabuError,
    xff::value::{self, Number, XffValue},
    Data,
};

use super::{deserialize_xff_number, deserialize_xff_text};

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

    // remove the file checksum ( 5 bytes )
    for _ in 0..5 {
        let _ = content.pop_front();
    }

    // EM check
    if content.len() > 0 {
        if content[0] == 25 {
            Ok(out)
        } else {
            Err(NabuError::TruncatedXFF(byte_pos.get(), 2))
        }
    } else {
        Err(NabuError::TruncatedXFF(byte_pos.get(), 2))
    }
}

fn check_file_checksum(content: &mut VecDeque<u8>, crc_table: &Crc32Table) -> bool {
    let mut tmp = content.clone();
    // pop EM
    tmp.pop_back();
    // pop checksum, 4 bytes
    let mut checksum: [u8; 4] = Default::default();
    // I am reading in the checksum backwards, so I reverse it here
    for i in 3..=0 {
        checksum[i] = tmp.pop_back().unwrap();
    }
    // pop CHK
    tmp.pop_back();

    let file_checksum: u32 = u32::from_le_bytes(checksum);
    if file_checksum == crc32_with_table(&tmp.into(), crc_table) {
        true
    } else {
        false
    }
}


#[inline]
fn deserialize_xff_value_checksum(content: &mut VecDeque<u8>, byte_pos: &Cell<usize>) -> Result<u32, NabuError> {
    // pop CHK
    let _ = content.pop_front();
    byte_pos.set(byte_pos.get() + 1);
    let mut checksum: [u8; 4] = Default::default();
    for i in 0..4 {
        checksum[i] = content.pop_front().ok_or(NabuError::TruncatedXFFValueChecksum(byte_pos.get(), 2))?;
    }
    let checksum = u32::from_le_bytes(checksum);
    Ok(checksum)
}

#[inline]
fn deserialize_xff_v2_value_length(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
) -> Result<usize, NabuError> {
    let (res, len) = deserialize_leb128_unsigned(content);
    byte_pos.set(byte_pos.get() + len as usize);
    Ok(res as usize)
}

pub fn deserialize_xff_v2_value(content: &mut VecDeque<u8>, byte_pos: &Cell<usize>, table: &Crc32Table) -> Result<XffValue, NabuError> {
    let value_type = content.pop_front().ok_or(NabuError::TruncatedXFFValue(byte_pos.get(), 2))?;
    match value_type{
        0 => {
            let _ = content.pop_front();
            byte_pos.set(byte_pos.get() + 1);
            //NUL
            return Ok(XffValue::Null);
        }/* 
        1 => deserialize_xff_v2_text(content, byte_pos, table),
        2 => deserialize_xff_v2_number(content, byte_pos, table),
        3 => deserialize_xff_v2_array(content, byte_pos, table),
        4 => deserialize_xff_v2_object(content, byte_pos, table),
        5 => deserialize_xff_v2_data(content, byte_pos, table), */
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
    if num_checksum != crc32_with_table(&data.clone().into(), table) {
        Err(NabuError::InvalidXFFValueChecksum(byte_pos.get(), 2))
    } else {
        byte_pos.set(byte_pos.get() - len);
        let out = deserialize_xff_number(&mut data, byte_pos);
        byte_pos.set(byte_pos.get() + len);
        byte_pos.set(byte_pos.get() + 6);
        out
    }
}
/* 
fn deserialize_xff_v2_array(content: &mut VecDeque<u8>, byte_pos: &Cell<usize>, table: &Crc32Table) -> Result<XffValue, NabuError> {
}

fn deserialize_xff_v2_object(content: &mut VecDeque<u8>, byte_pos: &Cell<usize>, table: &Crc32Table) -> Result<XffValue, NabuError> {

}
 */
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
    if num_checksum != crc32_with_table(&data.clone().into(), table) {
        Err(NabuError::InvalidXFFValueChecksum(byte_pos.get(), 2))
    } else {
        byte_pos.set(byte_pos.get() - len);
        let out = deserialize_xff_number(&mut data, byte_pos);
        byte_pos.set(byte_pos.get() + len);
        byte_pos.set(byte_pos.get() + 6);
        out
    }
}
