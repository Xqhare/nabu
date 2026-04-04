use crate::{
    error::{NabuError, Result},
    {CommandCharacter, XffValue},
};

/// Serializes XFF version 0 data.
///
/// # Errors
/// Errors if the data contains values not supported by v0.
pub fn serialize_xff_v0(data: Vec<XffValue>) -> Result<Vec<u8>> {
    let mut out: Vec<u8> = Vec::default();
    // Only true if the last pushed data was a command character
    let mut escape_open = false;
    // Version 0
    out.push(0);
    for value in data {
        match value {
            XffValue::String(s) => {
                let s = s.as_str();
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
                out.extend_from_slice(tmp.as_bytes());
                // ETX
                out.push(3);
            }
            XffValue::Data(d) => {
                escape_open = false;
                // DLE
                out.push(16);
                let len = d.len.to_le_bytes();
                out.extend_from_slice(&len[0..5]);
                out.extend_from_slice(&d.data);
                // DLE
                out.push(16);
            }
            XffValue::ArrayCmdChar(a) => {
                if escape_open {
                    // remove ending ESC
                    out.pop();
                } else {
                    // put starting ESC
                    out.push(27);
                }
                for char in a {
                    if let CommandCharacter::Escape = char {
                        // ESC needs to be ESC escaped
                        out.push(27);
                    }
                    out.push(char.as_u8());
                }
                // ESC
                out.push(27);
                escape_open = true;
            }
            XffValue::CommandCharacter(c) => {
                if escape_open {
                    // remove ending ESC
                    out.pop();
                } else {
                    // put starting ESC
                    out.push(27);
                }
                if let CommandCharacter::Escape = c {
                    // ESC needs to be ESC escaped
                    out.push(27);
                }
                out.push(c.as_u8());
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
