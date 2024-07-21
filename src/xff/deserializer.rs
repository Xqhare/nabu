use std::path::Path;

use crate::error::NabuError;

use super::value::{CommandCharacter, Data, XffValue};


pub fn deserialize_xff(path: &Path) -> Result<Vec<XffValue>, NabuError> {
    let content = std::fs::read(path);
    if content.is_err() {
        return Err(NabuError::from(content.unwrap_err()));
    } else {
        let mut ok_content = content.unwrap();
        let ver = ok_content.remove(0);
        match ver {
            0 => deserialize_xff_v0(&mut ok_content),
            _ => Err(NabuError::UnknownXFFVersion(ver)),
        }
    }
}

/// TODO:
///   - Add support for ECMA-404 numbers
fn deserialize_xff_v0(content: &mut Vec<u8>) -> Result<Vec<XffValue>, NabuError> {
    if content.len() == 0 {
        return Err(NabuError::InvalidXFF("Missing end of file marker".to_string()));
    }
    let mut out: Vec<XffValue> = Default::default();
    // byte 0 is the version, removed before
    let mut byte_pos: u64 = 1;
    while content.len() > 0 {
        let current_bytes = content.remove(0);
        byte_pos += 1;
        match current_bytes {
            2 => {
                // STX
                let mut tmp_string_binding = String::new();
                while content[0] != 3 {
                    let current_char = content.remove(0);
                    byte_pos += 1;
                    if current_char >= 48 && current_char <=57 || current_char == 150 {
                        // Maybe a number
                    };
                    match current_char {
                        8..=13 => {
                            // command characters
                            match current_char {
                                8 => {
                                    // Backspace
                                    tmp_string_binding.push('\x08')
                                },
                                9 => {
                                    // Horizontal Tab
                                    tmp_string_binding.push('\t')
                                },
                                10 => {
                                    // Line Feed
                                    tmp_string_binding.push('\n')
                                },
                                11 => {
                                    // Vertical Tab
                                    tmp_string_binding.push('\x0b')
                                },
                                12 => {
                                    // Form Feed
                                    tmp_string_binding.push('\x0c')
                                },
                                13 => {
                                    // Carriage Return
                                    tmp_string_binding.push('\r')
                                },
                                // Cannot happen
                                _ => {
                                    return Err(NabuError::InvalidXFF(format!("Invalid command character: {} at byte position: {}.", current_char, byte_pos)));
                                }
                            }
                        },
                        48..=57 | 150 => {
                        },
                        32..=126 | 128 | 130..=140 | 142 | 145..=156 | 158..=255 => {
                            tmp_string_binding.push(char::from_u32(current_char as u32).unwrap());
                        },
                        _ => {
                            return Err(NabuError::InvalidXFF(format!("Invalid ASCII character: {}.", current_char)));
                        }
                    }
                }
                if content[0] == 3 {
                    content.remove(0);
                    byte_pos += 1;
                    out.push(XffValue::String(tmp_string_binding));
                } else {
                    return Err(NabuError::InvalidXFF(format!("Missing end of transmission marker at byte position: {}", byte_pos)));
                }
            }
            16 => {
                // DLE
                let data_length: u64 = {
                    let mut tmp: Vec<u8> = Default::default();
                    for _ in 0..5 {
                        tmp.push(content.remove(0));
                        byte_pos += 1;
                    }
                    u64::from_be_bytes(tmp.try_into().unwrap())
                };
                let mut data = Vec::new();
                for _ in 0..data_length {
                    data.push(content.remove(0));
                    byte_pos += 1;
                }
                if content[0] == 16 {
                    content.remove(0);
                    byte_pos += 1;
                    out.push(XffValue::Data(Data { data, len: data_length as usize }));
                } else {
                    return Err(NabuError::InvalidXFF("Missing end of text marker".to_string()));
                }
            },
            25 => {
                // EM
                return Ok(out);
            },
            27 => {
                // ESC
                while content[0] != 27 || content[0] == 27 && content[1] == 27 {
                    let current_cmd_char = content.remove(0);
                    byte_pos += 1;
                    match current_cmd_char {
                        0..=32 | 127 | 160 | 173 => {
                            // ESC escaped check
                            if current_cmd_char == 27 {
                                if content[0] == 27 {
                                    content.remove(0);
                                    byte_pos += 1;
                                    out.push(XffValue::CommandCharacter(CommandCharacter::from(27)));
                                } else {
                                    return Err(NabuError::InvalidXFF(format!("Unescaped ESC at byte position: {}.", byte_pos)));
                                }
                            } else {
                                out.push(XffValue::CommandCharacter(CommandCharacter::from(current_cmd_char)));
                            }
                        },
                        _ => {
                            return Err(NabuError::InvalidXFF(format!("Invalid command character: {} at byte position: {}.", current_cmd_char, byte_pos)));
                        }
                    }
                }
            },
            _ => {
                return Err(NabuError::InvalidXFF(format!("Unknown byte: {} at byte position: {}.", current_bytes, byte_pos)));
            }
        }
    }
    Ok(out)
}
