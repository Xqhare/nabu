use std::collections::VecDeque;

use crate::{
    error::NabuError,
    xff::value::{CommandCharacter, Data, XffValue},
};

// ---------------------------------------------------
//                      LEGACY CODE
// ---------------------------------------------------

pub fn deserialize_xff_v0(content: &mut VecDeque<u8>) -> Result<XffValue, NabuError> {
    let xff_ver = 0;
    let mut out: Vec<XffValue> = Default::default();
    // version is byte 0; already match against and used but not removed, for performance, until now
    let _ = content.pop_front();
    let mut byte_pos: usize = 1;

    // debug; put true for debug, use --nocapture
    // this section takes 50ns
    let debug = false;
    let print_details = false;
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
            if print_details {
                // +1 for the remove below, its the position I am interested in
                println!("Main loop, byte pos is: {}", byte_pos + 1);
            }
            loop_amount += 1;
        }
        let current_byte = {
            if let Some(b) = content.pop_front() {
                byte_pos += 1;
                b
            } else {
                Err(NabuError::TruncatedXFF(byte_pos))?
            }
        };
        byte_pos += 1;
        match current_byte {
            2 => {
                // STX
                let now = std::time::Instant::now();
                let mut tmp_string_binding = String::new();
                while content[0] != 3 {
                    let current_char = {
                        if let Some(b) = content.pop_front() {
                            byte_pos += 1;
                            b
                        } else {
                            Err(NabuError::TruncatedXFF(byte_pos))?
                        }
                    };
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
                        return Err(NabuError::InvalidASCIIString(current_char, byte_pos, 0));
                    }
                }
                if content[0] == 3 {
                    let _ = content.pop_front();
                    byte_pos += 1;
                    out.push(XffValue::from((tmp_string_binding, xff_ver)));
                } else {
                    return Err(NabuError::MissingETX(byte_pos));
                }
                if debug {
                    let elapsed = now.elapsed();
                    if print_details {
                        println!("STX Elapsed: {:.2?}", elapsed);
                    }
                    stx_amount += 1;
                    stx_time_sum += elapsed;
                }
            }
            16 => {
                // DLE
                let now = std::time::Instant::now();
                // length, 5 bytes
                let data_length = u64::from_le_bytes([
                    content[0], content[1], content[2], content[3], content[4], 0, 0, 0,
                ]);
                let _ = content.drain(0..5);
                let data = content.drain(0..data_length as usize).collect::<Vec<u8>>();
                byte_pos += data_length as usize + 5;

                if content[0] == 16 {
                    let _ = content.pop_front();
                    byte_pos += 1;
                    out.push(XffValue::Data(Data {
                        len: data_length as usize,
                        data,
                    }));
                    if debug {
                        let elapsed = now.elapsed();
                        if print_details {
                            println!("DLE Elapsed: {:.2?}", elapsed);
                        }
                        dle_amount += 1;
                        dle_time_sum += elapsed;
                    }
                    continue;
                } else {
                    return Err(NabuError::MissingDLE(byte_pos));
                }
            }
            25 => {
                // EM
                if debug {
                    let elapsed = now_main.elapsed();
                    if print_details {
                        println!("Loop Elapsed: {:.2?}", elapsed);
                    }
                    loop_time_sum += elapsed;
                    if print_details || true {
                        if stx_amount > 0 {
                            println!("------------------------------------");
                            println!("STX Amount: {}", stx_amount);
                            println!("STX Time Sum: {:.2?}", stx_time_sum);
                            println!(
                                "STX Time Average: {:.2?}",
                                stx_time_sum / stx_amount.try_into().unwrap()
                            );
                        }
                        if dle_amount > 0 {
                            println!("------------------------------------");
                            println!("DLE Amount: {}", dle_amount);
                            println!("DLE Time Sum: {:.2?}", dle_time_sum);
                            println!(
                                "DLE Time Average: {:.2?}",
                                dle_time_sum / dle_amount.try_into().unwrap()
                            );
                        }
                        if cmd_amount > 0 {
                            println!("------------------------------------");
                            println!("CMD Amount: {}", cmd_amount);
                            println!("CMD Time Sum: {:.2?}", cmd_time_sum);
                            println!(
                                "CMD Time Average: {:.2?}",
                                cmd_time_sum / cmd_amount.try_into().unwrap()
                            );
                        }
                        if loop_amount > 0 {
                            println!("------------------------------------");
                            println!("Loop Amount: {}", loop_amount);
                            println!("Loop Time Sum: {:.2?}", loop_time_sum);
                            println!(
                                "Loop Time Average: {:.2?}",
                                loop_time_sum / loop_amount.try_into().unwrap()
                            );
                            println!("------------------------------------");
                        }
                    }
                    println!("------------------------------------");
                    println!("TOTALS:");
                    let t_time = stx_time_sum + dle_time_sum + cmd_time_sum + loop_time_sum;
                    let t_amount = stx_amount + dle_amount + cmd_amount + loop_amount;
                    let t_time_o_val = t_time / t_amount.try_into().unwrap();
                    println!("Total Time: {:.2?}", t_time);
                    println!("Total Values: {}", t_amount);
                    println!("Total Time over Values: {:.2?}", t_time_o_val);
                    println!("------------------------------------");
                }
                return Ok(out.into());
            }
            27 => {
                let now = std::time::Instant::now();
                // ESC
                loop {
                    if let Some(current_cmd_char) = content.pop_front() {
                        byte_pos += 1;
                        // ESC inverse check
                        if current_cmd_char != 27 {
                            let val = CommandCharacter::from_u8_checked(current_cmd_char);
                            if val.is_none() {
                                return Err(NabuError::InvalidASCIICommandCharacter(
                                    current_cmd_char,
                                    byte_pos,
                                ));
                            }
                            out.push(XffValue::CommandCharacter(val.unwrap()));
                            continue;
                        }
                        // Ending ESC
                        if content[0] != 27 {
                            break;
                        }
                        content.pop_front();
                        byte_pos += 1;
                        out.push(XffValue::CommandCharacter(CommandCharacter::from(27)));
                    } else {
                        // pop front returned None, truncation!
                        return Err(NabuError::TruncatedXFF(byte_pos));
                    };
                }
                if debug {
                    let elapsed = now.elapsed();
                    if print_details {
                        println!("ESC Elapsed: {:.2?}", elapsed);
                    }
                    cmd_amount += 1;
                    cmd_time_sum += elapsed;
                }
            }
            _ => {
                return Err(NabuError::InvalidXFFByte(current_byte, byte_pos, 0));
            }
        }
        if debug {
            let elapsed = now_main.elapsed();
            if print_details {
                println!("Loop Elapsed: {:.2?}", elapsed);
            }
            loop_time_sum += elapsed;
        }
    }
    // Premature EoF
    Err(NabuError::TruncatedXFF(byte_pos))
}
