use std::path::Path;

use crate::error::{NabuError, Result};

use super::value::XffValue;

/// Takes in a Vec of XffValues and serializes it into a byte vector
///
/// Determines the XFF version to use and calls the appropriate serializer
///
/// Because of version 0, the data argument has to be a vector, even if only one XffElement is
/// permissable as with version 1. In this case just "wrap" it in a vector, only the first element
/// is used.
///
/// # Arguments
/// * `path` - The path to the file to write
/// * `data` - The Vec of XffValues to write
/// * `ver` - The XFF version to use
///
/// # Errors
/// Returns IO errors when issues with reading the file from disk occur
pub fn serialize_xff(data: Vec<XffValue>, ver: u8) -> Result<Vec<u8>> {
    match ver {
        0 => serialize_xff_v0(data),
        1 => {
            if data.len() != 1 {
                return Err(NabuError::InvalidXFFVersion(data.into(), 1));
            }
            serialize_xff_v1(data)
        },
        _ => Err(NabuError::UnknownXFFVersion(ver)),
    }
}

fn serialize_xff_v1(data: Vec<XffValue>) -> Result<Vec<u8>> {
    let mut out: Vec<u8> = Default::default();
    // Version 1
    out.push(1);
    // now only one value is permissable
    for value in serialize_xff_v1_value(&data[0])? {
        out.push(value);
    }
    // EM
    out.push(25);
    Ok(out)
}

fn serialize_xff_v1_value(data: &XffValue) -> Result<Vec<u8>> {
    let mut out: Vec<u8> = Default::default();
    match data {
        XffValue::String(s) => {
            // first create the string
            let mut tmp_str: Vec<u8> = Default::default();
            for c in s.chars() {
                tmp_str.push(c as u8);
            }
            // now byte structure and push
            out.push(1);
            out.extend(encode_length(tmp_str.len()));
            out.extend(tmp_str);
            out.push(1);
        }
        XffValue::Number(n) => {
            // first create the string from the number
            let mut tmp_str: Vec<u8> = Default::default();
            let tmp = n.as_string();
            for entry in tmp.chars() {
                tmp_str.push(entry as u8);
            }
            // now byte structure and push
            out.push(2);
            out.extend(encode_length(tmp_str.len()));
            out.extend(tmp_str);
            out.push(2);
        }
        XffValue::Array(a) => {
            // create the array
            let mut array_bytes: Vec<u8> = Default::default();
            for value in a.iter() {
                array_bytes.extend(serialize_xff_v1_value(value)?);
                // RS separator
                array_bytes.push(30)
            }
            // byte structure and push
            out.push(3);
            out.extend(encode_length(array_bytes.len()));
            out.extend(array_bytes);
            out.push(3);
        }
        XffValue::Object(o) => {
            // create the object
            let mut object_bytes: Vec<u8> = Default::default();
            for (key, value) in o.iter() {
                // GS
                object_bytes.push(29);
                // key
                object_bytes.extend(serialize_xff_v1_value(&XffValue::String(key.clone()))?);
                // US
                object_bytes.push(31);
                // value
                out.extend(serialize_xff_v1_value(value)?);
                // Trailing GS
                object_bytes.push(29);
                // RS separator
                object_bytes.push(30)
            }
            // byte structure and push
            out.push(4);
            out.extend(encode_length(object_bytes.len()));
            out.extend(object_bytes);
            out.push(4);
        }
        XffValue::Data(d) => {
            out.push(5);
            out.extend(encode_length(d.len));
            out.extend(d.data.clone());
            out.push(5);
        }
        XffValue::Boolean(b) => {
            if *b {
                out.push(16);
            } else {
                out.push(17);
            }
        }
        XffValue::Null => {
            out.push(0);
        }
        _ => Err(NabuError::InvalidXFFVersion(data.clone(), 1))?
    }
    Ok(out)
}

fn encode_length(len: usize) -> Vec<u8> {
    if len <= 255 {
        let mut out: Vec<u8> = u8::from(1).to_le_bytes().to_vec();
        out.extend(len.to_le_bytes().to_vec());
        return out;
    } if len <= 65_535 {
        let mut out: Vec<u8> = u8::from(2).to_le_bytes().to_vec();
        out.extend(len.to_le_bytes().to_vec());
        return out;
    } if len <= 16_777_215 {
        let mut out: Vec<u8> = u8::from(3).to_le_bytes().to_vec();
        out.extend(len.to_le_bytes().to_vec());
        return out;
    } if len <= 4_294_967_295 {
        let mut out: Vec<u8> = u8::from(4).to_le_bytes().to_vec();
        out.extend(len.to_le_bytes().to_vec());
        return out;
    } if len <= 1_099_511_627_775 {
        let mut out: Vec<u8> = u8::from(5).to_le_bytes().to_vec();
        out.extend(len.to_le_bytes().to_vec());
        return out;
    } if len <= 281_474_976_710_655 {
        let mut out: Vec<u8> = u8::from(6).to_le_bytes().to_vec();
        out.extend(len.to_le_bytes().to_vec());
        return out;
    } else if len <= 72_057_594_037_927_35 {
        let mut out: Vec<u8> = u8::from(7).to_le_bytes().to_vec();
        out.extend(len.to_le_bytes().to_vec());
        return out;
    } else {
        let mut out: Vec<u8> = u8::from(8).to_le_bytes().to_vec();
        out.extend(len.to_le_bytes().to_vec());
        return out;
    }
}

fn serialize_xff_v0(data: Vec<XffValue>) -> Result<Vec<u8>> {
    let mut out: Vec<u8> = Default::default();
    // Only true if the last pushed data was a command character
    let mut escape_open = false;
    // Version 0
    out.push(0);
    for value in data {
        match value {
            XffValue::String(s) => {
                escape_open = false;
                // STX
                out.push(2);
                for c in s.chars() {
                    out.push(c as u8);
                }
                // ETX
                out.push(3);
            }
            XffValue::Number(n) => {
                escape_open = false;
                // STX
                out.push(2);
                let tmp = n.as_string();
                for entry in tmp.into_bytes() {
                    out.push(entry);
                }
                // ETX
                out.push(3);
            }
            XffValue::Data(d) => {
                escape_open = false;
                // DLE
                out.push(16);
                let len = d.len.to_le_bytes().to_vec();
                let len_bind: Vec<&u8> = len.iter().take(5).collect();
                for entry in len_bind {
                    out.push(*entry);
                }
                for entry in d.data.iter() {
                    out.push(*entry);
                }
                // DLE
                out.push(16);
            }
            XffValue::ArrayCmdChar(a) => {
                if escape_open {
                    // remove ending ESC
                    out.remove(out.len() - 1);
                } else {
                    // put starting ESC
                    out.push(27);
                }
                for char in a {
                    match char {
                        super::value::CommandCharacter::Escape => {
                            // ESC needs to be ESC escaped
                            out.push(27);
                            out.push(char.as_u8());
                        }
                        _ => {
                            out.push(char.as_u8());
                        }
                    }
                }
                // ESC
                out.push(27);
                escape_open = true;
            }
            XffValue::CommandCharacter(c) => {
                if escape_open {
                    // remove ending ESC
                    out.remove(out.len() - 1);
                } else {
                    // put starting ESC
                    out.push(27);
                }
                match c {
                    super::value::CommandCharacter::Escape => {
                        // ESC needs to be ESC escaped
                        out.push(27);
                        out.push(c.as_u8());
                    }
                    _ => {
                        out.push(c.as_u8());
                    }
                }
                // ESC
                out.push(27);
                escape_open = true;
            }
            _ => {
                return Err(NabuError::InvalidXFFValueForVersion(value, 0));
            }
        }
    }
    // EM
    out.push(25);
    Ok(out)
}

/// Writes a vector of bytes to a file
///
/// # Arguments
/// * `path` - The path to the file to write
/// * `data` - The vector of bytes to write
///
/// # Errors
/// Returns IO errors should issues with writing the file to disk arise
pub fn write_bytes_to_file(path: &Path, data: Vec<u8>) -> Result<()> {
    std::fs::write(path, data)?;
    Ok(())
}
