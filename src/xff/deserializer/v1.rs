
use std::{borrow::Borrow, cell::Cell, collections::{BTreeMap, VecDeque}, usize};

use crate::{error::NabuError, xff::value::{Number, XffValue}};

pub fn deserialize_xff_v1(contents: &mut VecDeque<u8>) -> Result<XffValue, NabuError> {
    // version is byte 0; already match against and used but not removed, for performance, until now
    let _ = contents.pop_front();
    let byte_pos: Cell<usize> = Cell::new(1);
    let out = deserialize_xff_v1_value(contents, byte_pos.borrow())?;
    if contents.len() > 0 {
        if contents[0] == 25 {
            Ok(out)
        } else {
            println!("FOUND YA");
            Err(NabuError::TruncatedXFF(byte_pos.get()))
        }
    } else {
        Err(NabuError::TruncatedXFF(byte_pos.get()))
    }
}

fn deserialize_xff_v1_value_length(content: &mut VecDeque<u8>, byte_pos: &Cell<usize>) -> Result<usize, NabuError> {
    let len_of_len_bytes = content.pop_front().ok_or(NabuError::TruncatedXFF(byte_pos.get()))?;
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
    Ok(usize::from_le_bytes([len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3], len_bytes[4], len_bytes[5], len_bytes[6], len_bytes[7]]))
}

fn deserialize_xff_v1_value(content: &mut VecDeque<u8>, byte_pos: &Cell<usize>) -> Result<XffValue, NabuError> {
    match content[0] {
        0 => {
            let _ = content.pop_front();
            byte_pos.set(byte_pos.get() + 1);

            Ok(XffValue::Null)
        },
        1 => {
            let _ = content.pop_front();
            byte_pos.set(byte_pos.get() + 1);
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

            let mut str_out: String = Default::default();
            while str_bytes.front().is_some() {
                let current_char = str_bytes.pop_front().unwrap();
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
                || current_char >= 158 {
                    str_out.push(char::from_u32(current_char as u32).unwrap());
                }else {
                    return Err(NabuError::InvalidASCIIString(current_char, byte_pos.get(), 1));
                }
            }
            Ok(XffValue::from(str_out))
        }
        2 => {
            let _ = content.pop_front();
            byte_pos.set(byte_pos.get() + 1);
            //NUM
            let len = deserialize_xff_v1_value_length(content, byte_pos)?;
            let num_bytes = content.drain(0..len).collect::<Vec<u8>>();
            byte_pos.set(byte_pos.get() + len);
            // check
            if content[0] != 24 {
                return Err(NabuError::MissingEV(byte_pos.get()));
            } else {
                let _ = content.pop_front();
                byte_pos.set(byte_pos.get() + 1);
            }

            let num_as_str = num_bytes.iter().map(|x| char::from_u32(*x as u32).unwrap()).collect::<String>();

            let check_usize = &num_as_str.parse::<usize>();
            if check_usize.is_ok() {
                Ok(XffValue::Number(Number::from(check_usize.as_ref().unwrap())))
            } else {
                let check_isize = &num_as_str.parse::<isize>();
                if check_isize.is_ok() {
                    Ok(XffValue::Number(Number::from(check_isize.as_ref().unwrap())))
                } else {
                    let check_float = &num_as_str.parse::<f64>();
                    if check_float.is_ok() {
                        Ok(XffValue::Number(Number::from(check_float.as_ref().unwrap())))
                    } else {
                        Err(NabuError::InvalidNumber(byte_pos.get(), num_as_str))
                    }
                }
            }

        }
        3 => {
            let _ = content.pop_front();
            byte_pos.set(byte_pos.get() + 1);
            //ARY

            let start_pos = byte_pos.get();
            let len = deserialize_xff_v1_value_length(content, byte_pos)?;

            let mut ary_bind: Vec<XffValue> = Default::default();

            ary_bind.push(deserialize_xff_v1_value(content, byte_pos)?);

            while content[0] != 3 && content.front().is_some() {
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
                }
            }

            // no trailing RS
            if content[0] == 24 {
                // closing ARY
                let _ = content.pop_front();
                byte_pos.set(byte_pos.get() + 1);
                if byte_pos.get() - start_pos != len {
                    return Err(NabuError::MissingEV(byte_pos.get()));
                }
                return Ok(XffValue::from(ary_bind));
            } else {
                return Err(NabuError::InvalidArray(byte_pos.get(), content[0]));
            }
        }
        4 => {
            let _ = content.pop_front();
            byte_pos.set(byte_pos.get() + 1);
            //OBJ
            
            let start_pos = byte_pos.get();
            let len = deserialize_xff_v1_value_length(content, byte_pos)?;

            let mut obj_bind: BTreeMap<String, XffValue> = Default::default();

            while content[0] != 4 && content.front().is_some() {
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
                }
            }

            // no trailing RS
            if content[0] == 24 {
                // closing ARY
                let _ = content.pop_front();
                byte_pos.set(byte_pos.get() + 1);
                if byte_pos.get() - start_pos != len {
                    return Err(NabuError::MissingEV(byte_pos.get()));
                }
                return Ok(XffValue::from(obj_bind));
            } else {
                return Err(NabuError::InvalidObject(byte_pos.get(), content[0]));
            }
        }
        5 => {
            let _ = content.pop_front();
            byte_pos.set(byte_pos.get() + 1);
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
            return Ok(XffValue::from(data));
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

fn deserialize_xff_v1_key_value(content: &mut VecDeque<u8>, byte_pos: &Cell<usize>) -> Result<(String, XffValue), NabuError> {
    // GS
    if content[0] != 29 {
        return Err(NabuError::InvalidObject(byte_pos.get(), content[0]));
    } else {
        let _ = content.pop_front();
        byte_pos.set(byte_pos.get() + 1);

        let mut key: String = Default::default();
        while content[0] != 31 && content.front().is_some() {
            key.push(content.pop_front().unwrap() as char);
            byte_pos.set(byte_pos.get() + 1);
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

                return Ok((key, value));
            }
        }
    }
}
