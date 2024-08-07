use std::{path::Path, rc::Rc, sync::Mutex};

use crate::error::NabuError;

use super::value::{CommandCharacter, Data, Number, XffValue};

/// Reads the content of a XFF file and returns a Vec
///
/// Reads the first byte of the file to determine the version and then calls the appropriate deserializer for the version
///
/// # Arguments
/// * `path` - The path to the file to read
///
/// # Errors
/// Returns IO errors when issues with reading the file from disk occur
/// Also returns `NabuError::UnknownXFFVersion` when the version is higher than the current highest version of the XFF format
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
///   - Add support for ECMA-404 numbers -> DONE
///   - Add better support for ECMA-404 numbers
///   - OPTIMISE
fn deserialize_xff_v0(content: &mut Vec<u8>) -> Result<Vec<XffValue>, NabuError> {
    if content.len() == 0 {
        return Err(NabuError::InvalidXFF(
            "Missing end of file marker".to_string(),
        ));
    }
    let mut out: Vec<XffValue> = Default::default();
    // byte 0 is the version, removed before, the +1 again for normal counting is not done
    // here, remember that if counting bytes in files to find an error!
    let mut byte_pos: usize = 1;
    let mut stx_amount = usize::MIN;
    let mut stx_time_sum: std::time::Duration = std::time::Duration::ZERO;
    while content.len() > 0 {
        let current_bytes = content.remove(0);
        byte_pos += 1;
        println!("Main loop, byte pos is: {}", byte_pos);
        match current_bytes {
            2 => {
                // STX
                let now = std::time::Instant::now();
                let mut tmp_string_binding = String::new();
                let mut is_number = true;
                while content[0] != 3 {
                    let current_char = content.remove(0);
                    byte_pos += 1;
                    if current_char >= 8 && current_char <= 13 {
                        is_number = false;
                        // command characters
                        match current_char {
                            8 => {
                                // Backspace
                                tmp_string_binding.push('\x08')
                            }
                            9 => {
                                // Horizontal Tab
                                tmp_string_binding.push('\t')
                            }
                            10 => {
                                // Line Feed
                                tmp_string_binding.push('\n')
                            }
                            11 => {
                                // Vertical Tab
                                tmp_string_binding.push('\x0b')
                            }
                            12 => {
                                // Form Feed
                                tmp_string_binding.push('\x0c')
                            }
                            13 => {
                                // Carriage Return
                                tmp_string_binding.push('\r')
                            }
                            _ => {
                                unreachable!()
                            }
                        }
                    }
                    if is_number {
                        // Only valid ASCII number characters
                        if current_char >= 48 && current_char <= 57
                            || current_char >= 43 && current_char <= 46
                            || current_char == 69 || current_char == 101
                        {
                            let mut tmp_number_binding = String::from_utf8(vec![current_char]).expect("Windows 1252 is a subset of utf8");
                            while is_number {
                                if content[0] == 3 {
                                    tmp_string_binding = tmp_number_binding;
                                    is_number = false;
                                    break;
                                }
                                let current_char = content.remove(0);
                                byte_pos += 1;
                                if current_char >= 48 && current_char <= 57
                                    || current_char >= 43 && current_char <= 46
                                    || current_char == 69 || current_char == 101
                                {
                                    tmp_number_binding
                                        .push(char::from_u32(current_char as u32).unwrap());
                                } else {
                                    tmp_string_binding = tmp_number_binding;
                                    is_number = false;
                                    break;
                                }
                            }
                        }
                    }
                    // All valid ASCII string characters
                    if current_char >= 32 && current_char <= 126
                        || current_char == 128
                        || current_char >= 130 && current_char <= 140
                        || current_char == 142
                        || current_char >= 145 && current_char <= 156
                        || current_char >= 158
                    {
                        is_number = false;
                        tmp_string_binding.push(char::from_u32(current_char as u32).unwrap());
                    } else {
                        return Err(NabuError::InvalidXFF(format!(
                            "Invalid ASCII character: {}.",
                            current_char
                        )));
                    }
                }
                if content[0] == 3 {
                    content.remove(0);
                    byte_pos += 1;
                    if is_number {
                        if tmp_string_binding.parse::<usize>().is_ok() {
                        out.push(XffValue::Number(Number::Unsigned(
                            tmp_string_binding.parse::<usize>().unwrap(),
                        )));
                        } else if tmp_string_binding.parse::<isize>().is_ok() {
                            out.push(XffValue::Number(Number::Integer(
                                tmp_string_binding.parse::<isize>().unwrap(),
                            )));
                        } else if tmp_string_binding.parse::<f64>().is_ok() {
                            out.push(XffValue::Number(Number::Float(
                                tmp_string_binding.parse::<f64>().unwrap(),
                            )));
                        } else {
                            Err(NabuError::InvalidXFF(format!(
                                "Unable to convert correct number syntax: '{}' to a number.",
                                tmp_string_binding
                            )))?
                        };
                    } else {
                        out.push(XffValue::String(tmp_string_binding));
                    }
                } else {
                    return Err(NabuError::InvalidXFF(format!(
                        "Missing end of transmission marker at byte position: {}",
                        byte_pos
                    )));
                }
                let elapsed = now.elapsed();
                println!("STX Elapsed: {:.2?}", elapsed);
                stx_amount += 1;
                stx_time_sum += elapsed;
            }
            16 => {
                let now = std::time::Instant::now();
                // DLE
                let data_length: u64 = {
                    // My first real array!
                    let mut tmp: [u8; 8] = Default::default();
                    for n in 0..5 {
                        tmp[n] = content.remove(0);
                        byte_pos += 1;
                    }
                    u64::from_le_bytes(tmp)
                };
                let mut data = Vec::new();
                for _ in 0..data_length {
                    data.push(content.remove(0));
                    byte_pos += 1;
                }
                if content[0] == 16 {
                    content.remove(0);
                    byte_pos += 1;
                    out.push(XffValue::Data(Data {
                        data,
                        len: data_length as usize,
                    }));
                } else {
                    return Err(NabuError::InvalidXFF(
                        "Missing end of text marker".to_string(),
                    ));
                }
                let elapsed = now.elapsed();
                println!("DLE Elapsed: {:.2?}", elapsed);
            }
            25 => {
                // EM
                return Ok(out);
            }
            27 => {
                let now = std::time::Instant::now();
                // ESC
                loop {
                    let current_cmd_char = content.remove(0);
                    byte_pos += 1;
                    // ESC inverse check
                    if current_cmd_char != 27 {
                        let val = CommandCharacter::from_u8_checked(
                            current_cmd_char,
                        );
                        if val.is_none() {
                            return Err(NabuError::InvalidXFF(format!(
                                "Invalid command character: {} at byte position: {}.",
                                current_cmd_char, byte_pos
                            )));
                        }
                        out.push(XffValue::CommandCharacter(val.unwrap()));
                        continue;
                    }
                    // Ending ESC
                    if content[0] != 27 {
                        break;
                    } 
                    content.remove(0);
                    byte_pos += 1;
                    out.push(XffValue::CommandCharacter(CommandCharacter::from(
                        27,
                    )));
                }
                let elapsed = now.elapsed();
                println!("ESC Elapsed: {:.2?}", elapsed);
            }
            _ => {
                return Err(NabuError::InvalidXFF(format!(
                    "Unknown byte: {} at byte position: {}.",
                    current_bytes, byte_pos
                )));
            }
        }
    };
    println!("DODODOD");
    println!("STX Amount: {}", stx_amount);
    println!("STX Time Sum: {:.2?}", stx_time_sum);
    println!("STX Time Average: {:.2?}", stx_time_sum / stx_amount.try_into().unwrap());
    Ok(out)
}
