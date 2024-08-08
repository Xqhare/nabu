use std::{collections::VecDeque, path::Path};

use crate::error::NabuError;

use super::value::{CommandCharacter, Data, XffValue};

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
///   - Add better support for ECMA-404 numbers, I just moved the poor mans parser into
///   XffValue::from(String)
///   - OPTIMISE -> DONE
fn deserialize_xff_v0(contents: &mut Vec<u8>) -> Result<Vec<XffValue>, NabuError> {
    if contents.len() == 0 {
        return Err(NabuError::InvalidXFF(
            "Missing end of file marker".to_string(),
        ));
    }
    let mut content: VecDeque<u8> = contents.drain(..).collect();
    let mut out: Vec<XffValue> = Default::default();
    // version is byte 0
    let mut byte_pos: usize = 1;

    let debug = false;
    let mut loop_amount = usize::MIN;
    let mut loop_time_sum: std::time::Duration = std::time::Duration::ZERO;
    let mut stx_amount = usize::MIN;
    let mut stx_time_sum: std::time::Duration = std::time::Duration::ZERO;
    let mut dle_amount = usize::MIN;
    let mut dle_time_sum: std::time::Duration = std::time::Duration::ZERO;
    let mut cmd_amount = usize::MIN;
    let mut cmd_time_sum: std::time::Duration = std::time::Duration::ZERO;
    
    while content.len() > 0 {
        let now_main = std::time::Instant::now();
        if debug {
            // +1 for the remove below, its the position I am interested in
            println!("Main loop, byte pos is: {}", byte_pos + 1);
            loop_amount += 1;
        }
        let current_bytes = content.pop_front().unwrap();
        byte_pos += 1;
        match current_bytes {
            2 => {
                // STX
                let now = std::time::Instant::now();
                let mut tmp_string_binding = String::new();
                while content[0] != 3 {
                    let current_char = content.pop_front().unwrap();
                    byte_pos += 1;
                    if current_char >= 8 && current_char <= 13 {
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
                    // All valid ASCII string characters
                    if current_char >= 32 && current_char <= 126
                        || current_char == 128
                        || current_char >= 130 && current_char <= 140
                        || current_char == 142
                        || current_char >= 145 && current_char <= 156
                        || current_char >= 158
                    {
                        tmp_string_binding.push(char::from_u32(current_char as u32).unwrap());
                    } else {
                        return Err(NabuError::InvalidXFF(format!(
                            "Invalid ASCII character: {}.",
                            current_char
                        )));
                    }
                }
                if content[0] == 3 {
                    let _ = content.pop_front().unwrap();
                    byte_pos += 1;
                    out.push(XffValue::from(tmp_string_binding));
                } else {
                    return Err(NabuError::InvalidXFF(format!(
                        "Missing end of transmission marker at byte position: {}",
                        byte_pos
                    )));
                }
                if debug {
                   let elapsed = now.elapsed();
                    println!("STX Elapsed: {:.2?}", elapsed);
                    stx_amount += 1;
                    stx_time_sum += elapsed;
                }
            }
            16 => {
                let now = std::time::Instant::now();
                // DLE
                let data_length: u64 = {
                    // My first real array!
                    let tmp: Vec<u8> = content.drain(0..5).collect::<Vec<u8>>();
                    byte_pos += 5;
                    let arr = [tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], 0, 0, 0];
                    u64::from_le_bytes(arr)
                };
                let data = content.drain(0..data_length as usize).collect::<Vec<u8>>();
                byte_pos += data_length as usize;
                if content[0] == 16 {
                    content.pop_front().unwrap();
                    byte_pos += 1;
                    out.push(XffValue::Data(Data {
                        len: data_length as usize,
                        data,
                    }));
                } else {
                    return Err(NabuError::InvalidXFF(
                        "Missing end of text marker".to_string(),
                    ));
                }
                if debug {
                    let elapsed = now.elapsed();
                    println!("DLE Elapsed: {:.2?}", elapsed);
                    dle_amount += 1;
                    dle_time_sum += elapsed;
                }
            }
            25 => {
                // EM
                if debug {
                    let elapsed = now_main.elapsed();
                    println!("Loop Elapsed: {:.2?}", elapsed);
                    loop_time_sum += elapsed;
                    if stx_amount > 0 {
                        println!("------------------------------------");
                        println!("STX Amount: {}", stx_amount);
                        println!("STX Time Sum: {:.2?}", stx_time_sum);
                        println!("STX Time Average: {:.2?}", stx_time_sum / stx_amount.try_into().unwrap());
                    }
                    if dle_amount > 0 {
                        println!("------------------------------------");
                        println!("DLE Amount: {}", dle_amount);
                        println!("DLE Time Sum: {:.2?}", dle_time_sum);
                        println!("DLE Time Average: {:.2?}", dle_time_sum / dle_amount.try_into().unwrap());
                    }
                    if cmd_amount > 0 {
                        println!("------------------------------------");
                        println!("CMD Amount: {}", cmd_amount);
                        println!("CMD Time Sum: {:.2?}", cmd_time_sum);
                        println!("CMD Time Average: {:.2?}", cmd_time_sum / cmd_amount.try_into().unwrap());
                    }
                    if loop_amount > 0 {
                        println!("------------------------------------");
                        println!("Loop Amount (Total Value Amount): {}", loop_amount);
                        println!("Loop Time Sum: {:.2?}", loop_time_sum);
                        println!("Loop Time Average: {:.2?}", loop_time_sum / loop_amount.try_into().unwrap());
                        println!("------------------------------------");
                    }
                }
                return Ok(out);
            }
            27 => {
                let now = std::time::Instant::now();
                // ESC
                loop {
                    let current_cmd_char = content.pop_front().unwrap();
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
                    content.pop_front().unwrap();
                    byte_pos += 1;
                    out.push(XffValue::CommandCharacter(CommandCharacter::from(
                        27,
                    )));
                }
                if debug {
                    let elapsed = now.elapsed();
                    println!("ESC Elapsed: {:.2?}", elapsed);
                    cmd_amount += 1;
                    cmd_time_sum += elapsed;
                }
            }
            _ => {
                return Err(NabuError::InvalidXFF(format!(
                    "Unknown byte: {} at byte position: {}.",
                    current_bytes, byte_pos
                )));
            }
        }
        if debug {
            let elapsed = now_main.elapsed();
            println!("Loop Elapsed: {:.2?}", elapsed);
            loop_time_sum += elapsed;
        }
    };
    // If a length of 0 is ever read, its an error, files have to end with EM and start with
    // version, so 2 bytes total
    Err(NabuError::InvalidXFF(format!(
        "Missing end of transmission marker at byte position: {}.",
        byte_pos,
    )))
}

