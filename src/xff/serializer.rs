use std::path::Path;

use crate::error::NabuError;

use super::value::XffValue;

pub fn serialize_xff(path: &Path, data: Vec<XffValue>, ver: u8) -> Result<(), NabuError> {
    match ver {
        0 => serialize_xff_v0(path, data),
        _ => Err(NabuError::UnknownXFFVersion(ver)),
    }
}

fn serialize_xff_v0(path: &Path, data: Vec<XffValue>) -> Result<(), NabuError>{
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
            },
            XffValue::Number(n) => {
                escape_open = false;
                // STX
                out.push(2);
                for entry in n.as_u8() {
                    out.push(entry);
                }
                // ETX
                out.push(3);
            },
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
            },
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
            },
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
            },
        }
    }
    // EM
    out.push(25);
    std::fs::write(path, out)?;
    Ok(())
}
