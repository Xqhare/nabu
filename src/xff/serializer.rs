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
    // Version 0
    out.push(0);
    for value in data {
        match value {
            XffValue::String(s) => {
                // STX
                out.push(2);
                for c in s.chars() {
                    out.push(c as u8);
                }
                // ETX
                out.push(3);
            },
            XffValue::Number(n) => {
                // STX
                out.push(2);
                for entry in n.as_u8() {
                    out.push(entry);
                }
                // ETX
                out.push(3);
            },
            XffValue::Data(d) => {
                // DLE
                out.push(16);
                let len = d.len.to_be_bytes().to_vec();
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
            XffValue::CommandCharacter(c) => {
                match c {
                    super::value::CommandCharacter::Escape => {
                        // ESC
                        out.push(27);
                        // ESC needs to be ESC escaped
                        out.push(27);
                        out.push(c.as_u8());
                        // ESC
                        out.push(27);
                    }
                    _ => {
                        // ESC
                        out.push(27);
                        out.push(c.as_u8());
                        // ESC
                        out.push(27);
                    }
                }
            },
        }
    }
    // EM
    out.push(25);
    std::fs::write(path, out)?;
    Ok(())
}
