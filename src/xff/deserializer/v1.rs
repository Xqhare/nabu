use std::{
    borrow::Borrow,
    cell::Cell,
    collections::{BTreeMap, VecDeque},
};

use crate::{XffValue, error::NabuError};

use super::{
    deserialize_xff_data, deserialize_xff_key_value, deserialize_xff_number, deserialize_xff_text,
};

use nemesis::NemesisResultExt;
use crate::error::Result;

/// Deserializes XFF version 1 content.
///
/// # Errors
/// Errors if the content is malformed or truncated according to v1 specification.
pub fn deserialize_xff_v1(contents: &mut VecDeque<u8>) -> Result<XffValue> {
    // version is byte 0;
    let byte_pos: Cell<usize> = Cell::new(1);
    let out = deserialize_xff_v1_value(contents, byte_pos.borrow())
        .add_source("nabu::xff::deserializer::v1")?;
    if contents.is_empty() {
        Err(NabuError::TruncatedXFF(byte_pos.get(), 1).into())
    } else if contents[0] == 25 && contents.len() == 1 {
        Ok(out)
    } else {
        Err(NabuError::TruncatedXFF(byte_pos.get(), 1).into())
    }
}

/// Deserializes a single XFF v1 value.
///
/// # Errors
/// Errors if the value is malformed or truncated.
pub fn deserialize_xff_v1_value(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
) -> Result<XffValue> {
    if content.is_empty() {
        return Err(NabuError::TruncatedXFFValue(byte_pos.get(), 1).into());
    }
    let cur = content.pop_front().unwrap();
    byte_pos.set(byte_pos.get() + 1);
    match cur {
        0 => Ok(XffValue::Null),
        1 => deserialize_xff_v1_text(content, byte_pos),
        2 => deserialize_xff_v1_number(content, byte_pos),
        3 => deserialize_xff_v1_array(content, byte_pos),
        4 => deserialize_xff_v1_object(content, byte_pos),
        5 => deserialize_xff_v1_data(content, byte_pos),
        16 => Ok(XffValue::from(true)),
        17 => Ok(XffValue::from(false)),
        _ => Err(NabuError::InvalidXFFByte(cur, byte_pos.get(), 1).into()),
    }
}

fn deserialize_xff_v1_text(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
) -> Result<XffValue> {
    //TXT

    // reading length first
    let len = deserialize_xff_v1_value_length(content, byte_pos)?;
    // drain the string from the content
    if content.len() < len {
        return Err(NabuError::TruncatedXFF(byte_pos.get(), 1).into());
    }
    let mut str_bytes = content.drain(0..len).collect::<VecDeque<u8>>();
    let out = deserialize_xff_text(&mut str_bytes, byte_pos, 1);
    // check
    if content[0] != 24 {
        return Err(NabuError::MissingEV(byte_pos.get()).into());
    }
    let _ = content.pop_front();
    byte_pos.set(byte_pos.get() + 1);
    out
}

fn deserialize_xff_v1_array(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
) -> Result<XffValue> {
    //ARY

    let len = deserialize_xff_v1_value_length(content, byte_pos)?;

    let mut ary_bind: Vec<XffValue> = Vec::default();

    if len != 0 {
        ary_bind.push(deserialize_xff_v1_value(content, byte_pos)?);
    }

    while content.front().is_some() && content[0] != 24 {
        if content[0] == 30 {
            if content.len() > 1 && content[1] == 24 {
                // closing ARY
                let _ = content.pop_front();
                let _ = content.pop_front();
                byte_pos.set(byte_pos.get() + 2);
                return Ok(XffValue::from(ary_bind));
            }
            let _ = content.pop_front();
            byte_pos.set(byte_pos.get() + 1);
            // another value
            ary_bind.push(deserialize_xff_v1_value(content, byte_pos)?);
        } else {
            break;
        }
    }

    // no trailing RS
    if !content.is_empty() && content[0] == 24 {
        // closing ARY
        let _ = content.pop_front();
        byte_pos.set(byte_pos.get() + 1);

        Ok(XffValue::from(ary_bind))
    } else {
        Err(NabuError::InvalidArray(byte_pos.get(), content[0], 1).into())
    }
}

fn deserialize_xff_v1_object(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
) -> Result<XffValue> {
    //OBJ

    let len = deserialize_xff_v1_value_length(content, byte_pos)?;

    let mut obj_bind: BTreeMap<String, XffValue> = BTreeMap::default();

    if len != 0 {
        while content.front().is_some() && content[0] != 24 {
            let (key, value) = deserialize_xff_key_value(content, byte_pos, 1)?;
            obj_bind.insert(key, value);
            if content[0] == 30 {
                if content.len() > 1 && content[1] == 24 {
                    // closing OBJ
                    let _ = content.pop_front();
                    let _ = content.pop_front();
                    byte_pos.set(byte_pos.get() + 2);
                    return Ok(XffValue::from(obj_bind));
                }
                let _ = content.pop_front();
                byte_pos.set(byte_pos.get() + 1);
                // another key value pair
                continue;
            }
            break;
        }
    }

    // no trailing RS
    if !content.is_empty() && content[0] == 24 {
        // closing ARY
        let _ = content.pop_front();
        byte_pos.set(byte_pos.get() + 1);

        Ok(XffValue::from(obj_bind))
    } else {
        Err(NabuError::InvalidObject(byte_pos.get(), content[0], 1).into())
    }
}

fn deserialize_xff_v1_data(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
) -> Result<XffValue> {
    let len = deserialize_xff_v1_value_length(content, byte_pos)?;
    let data = deserialize_xff_data(content, byte_pos, len);
    if content[0] != 24 {
        return Err(NabuError::MissingEV(byte_pos.get()).into());
    }
    let _ = content.pop_front();
    byte_pos.set(byte_pos.get() + 1);
    Ok(data)
}

fn deserialize_xff_v1_number(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
) -> Result<XffValue> {
    let len = deserialize_xff_v1_value_length(content, byte_pos)?;
    let mut num_bytes = content.drain(0..len).collect::<VecDeque<u8>>();

    // check
    if content[0] != 24 {
        return Err(NabuError::MissingEV(byte_pos.get()).into());
    }
    let _ = content.pop_front();
    byte_pos.set(byte_pos.get() + 1);
    deserialize_xff_number(&mut num_bytes, byte_pos, 1)
}

fn deserialize_xff_v1_value_length(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
) -> std::result::Result<usize, NabuError> {
    let len_of_len_bytes = content
        .pop_front()
        .ok_or(NabuError::TruncatedXFF(byte_pos.get(), 1))?;
    byte_pos.set(byte_pos.get() + 1);
    let len_of_len = len_of_len_bytes;
    if len_of_len > 8 {
        return Err(NabuError::InvalidXFFValueLength(usize::from(len_of_len), 1));
    } else if content.len() < len_of_len as usize {
        return Err(NabuError::XFFValueLengthTooLong(
            usize::from(len_of_len),
            byte_pos.get(),
            1,
        ));
    }
    let mut len_bytes = content.drain(0..len_of_len as usize).collect::<Vec<u8>>();
    byte_pos.set(byte_pos.get() + len_of_len as usize);
    if len_bytes.len() < 8 {
        len_bytes.resize(8, 0);
    }
    Ok(usize::from_le_bytes([
        len_bytes[0],
        len_bytes[1],
        len_bytes[2],
        len_bytes[3],
        len_bytes[4],
        len_bytes[5],
        len_bytes[6],
        len_bytes[7],
    ]))
}
