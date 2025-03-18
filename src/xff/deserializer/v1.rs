use std::{
    borrow::Borrow,
    cell::Cell,
    collections::{BTreeMap, VecDeque},
    usize,
};

use crate::{
    error::NabuError,
    xff::value::{Number, XffValue},
    Data,
};

use super::{deserialize_xff_number, deserialize_xff_text};

pub fn deserialize_xff_v1(contents: &mut VecDeque<u8>) -> Result<XffValue, NabuError> {
    // version is byte 0; 
    let byte_pos: Cell<usize> = Cell::new(1);
    let out = deserialize_xff_v1_value(contents, byte_pos.borrow())?;
    if contents.len() > 0 {
        if contents[0] == 25 {
            Ok(out)
        } else {
            Err(NabuError::TruncatedXFF(byte_pos.get()))
        }
    } else {
        Err(NabuError::TruncatedXFF(byte_pos.get()))
    }
}

fn deserialize_xff_v1_value_length(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
) -> Result<usize, NabuError> {
    let len_of_len_bytes = content
        .pop_front()
        .ok_or(NabuError::TruncatedXFF(byte_pos.get()))?;
    byte_pos.set(byte_pos.get() + 1);
    let len_of_len = u8::from_le_bytes([len_of_len_bytes]);
    if len_of_len > 8 {
        return Err(NabuError::InvalidXFFValueLength(len_of_len.into()));
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

fn deserialize_xff_v1_value(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
) -> Result<XffValue, NabuError> {
    match content[0] {
        0 => {
            let _ = content.pop_front();
            byte_pos.set(byte_pos.get() + 1);

            Ok(XffValue::Null)
        }
        1 => {
            let _ = content.pop_front();
            byte_pos.set(byte_pos.get() + 1);

            deserialize_xff_v1_text(content, byte_pos)
        }
        2 => {
            let _ = content.pop_front();
            byte_pos.set(byte_pos.get() + 1);

            deserialize_xff_v1_number(content, byte_pos)
        }
        3 => {
            let _ = content.pop_front();
            byte_pos.set(byte_pos.get() + 1);

            deserialize_xff_v1_array(content, byte_pos)
        }
        4 => {
            let _ = content.pop_front();
            byte_pos.set(byte_pos.get() + 1);

            deserialize_xff_v1_object(content, byte_pos)
        }
        5 => {
            let _ = content.pop_front();
            byte_pos.set(byte_pos.get() + 1);
            
            deserialize_xff_v1_data(content, byte_pos)
        }
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

fn deserialize_xff_v1_text(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
) -> Result<XffValue, NabuError> {
    //TXT

    // reading length first
    let len = deserialize_xff_v1_value_length(content, byte_pos)?;
    // drain the string from the content
    let mut str_bytes = content.drain(0..len).collect::<VecDeque<u8>>();
    byte_pos.set(byte_pos.get() + len);
    // check
    if content[0] != 24 {
        return Err(NabuError::MissingEV(byte_pos.get()));
    } else {
        let _ = content.pop_front();
        byte_pos.set(byte_pos.get() + 1);
    }
    
    deserialize_xff_text(&mut str_bytes, byte_pos)
}

fn deserialize_xff_v1_array(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
) -> Result<XffValue, NabuError> {
    //ARY

    let len = deserialize_xff_v1_value_length(content, byte_pos)?;

    let mut ary_bind: Vec<XffValue> = Default::default();

    if len != 0 {
        ary_bind.push(deserialize_xff_v1_value(content, byte_pos)?);
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
                ary_bind.push(deserialize_xff_v1_value(content, byte_pos)?);
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
        return Err(NabuError::InvalidArray(byte_pos.get(), content[0]));
    }
}

fn deserialize_xff_v1_object(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
) -> Result<XffValue, NabuError> {
    //OBJ

    let len = deserialize_xff_v1_value_length(content, byte_pos)?;

    let mut obj_bind: BTreeMap<String, XffValue> = Default::default();

    if len != 0 {
        while content[0] != 24 && content.front().is_some() {
            let (key, value) = deserialize_xff_v1_key_value(content, byte_pos)?;
            obj_bind.insert(key, value);
            if content[0] == 30 {
                if content[1] == 24 {
                    // closing OBJ
                    let _ = content.pop_front();
                    let _ = content.pop_front();
                    byte_pos.set(byte_pos.get() + 2);
                    return Ok(XffValue::from(obj_bind));
                } else {
                    let _ = content.pop_front();
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
        return Err(NabuError::InvalidObject(byte_pos.get(), content[0]));
    }
}

fn deserialize_xff_v1_data(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
) -> Result<XffValue, NabuError> {
    //DAT
    let len = deserialize_xff_v1_value_length(content, byte_pos)?;
    let data = content.drain(0..len).collect::<Vec<u8>>();
    byte_pos.set(byte_pos.get() + len);
    if content[0] != 24 {
        return Err(NabuError::MissingEV(byte_pos.get()));
    } else {
        let _ = content.pop_front();
        byte_pos.set(byte_pos.get() + 1);
    }
    return Ok(XffValue::from(Data::from(data)));
}

fn deserialize_xff_v1_number(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
) -> Result<XffValue, NabuError> {
    let len = deserialize_xff_v1_value_length(content, byte_pos)?;
    let mut num_bytes = content.drain(0..len).collect::<VecDeque<u8>>();
    byte_pos.set(byte_pos.get() + len);

    // check
    if content[0] != 24 {
        return Err(NabuError::MissingEV(byte_pos.get()));
    } else {
        let _ = content.pop_front();
        byte_pos.set(byte_pos.get() + 1);
    }
    deserialize_xff_number(&mut num_bytes, byte_pos)
}

fn deserialize_xff_v1_key_value(
    content: &mut VecDeque<u8>,
    byte_pos: &Cell<usize>,
) -> Result<(String, XffValue), NabuError> {
    // GS
    if content[0] != 29 {
        return Err(NabuError::InvalidObject(byte_pos.get(), content[0]));
    } else {
        let _ = content.pop_front();
        byte_pos.set(byte_pos.get() + 1);

        let mut key_bytes: VecDeque<u8> = Default::default();
        while content[0] != 31 && content.front().is_some() {
            key_bytes.push_back(content.pop_front().unwrap());
            byte_pos.set(byte_pos.get() + 1);
        }
        let key_bind = deserialize_xff_v1_value(&mut key_bytes, byte_pos)?;
        if !key_bind.is_string() {
            return Err(NabuError::InvalidKey(byte_pos.get(), key_bind));
        }

        // US
        if content[0] != 31 {
            return Err(NabuError::InvalidObject(byte_pos.get(), content[0]));
        } else {
            let _ = content.pop_front();
            byte_pos.set(byte_pos.get() + 1);

            let value = deserialize_xff_v1_value(content, byte_pos)?;
            // Trailing GS
            if content[0] != 29 {
                return Err(NabuError::InvalidObject(byte_pos.get(), content[0]));
            } else {
                let _ = content.pop_front();
                byte_pos.set(byte_pos.get() + 1);

                return Ok((
                    key_bind.into_string().expect("Checked for String above!"),
                    value,
                ));
            }
        }
    }
}
