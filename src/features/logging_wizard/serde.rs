use std::{collections::BTreeMap, path::Path};
use crate::{XFF_VERSION, serde::read, xff::{value::{XffValue, CommandCharacter}, serializer::{serialize_xff, write_bytes_to_file}}, error::NabuError, features::logging_wizard::{LoggingWizard, Log, LogData}};

/// Encodes a Vec of logs into bytes
/// Writes an entirely new xff file to the given path
///
/// # Arguments
/// * `path` - The path to the file to write
/// * `data` - The data to write
pub fn write_log_wizard(path: &Path, data: &Vec<Log>) -> Result<(), NabuError> {
    let byte_data = logs_to_bytes(data)?;
    write_bytes_to_file(path, byte_data)
}


/// Appends a Vec of logs to an existing xff file
/// Drops the last byte of the file
///
/// # Arguments
/// * `path` - The path to the file to write
/// * `data` - The data to write
pub fn append_to_log_wizard(path: &Path, data: &Vec<Log>) -> Result<(), NabuError> {
    let mut byte_data = logs_to_bytes(data)?;
    if !path.exists() {
        std::fs::write(path, byte_data)?;
        Ok(())
    } else {
        let mut file_as_bytes: Vec<u8> = std::fs::read(path)?;
        // Dropping the last byte, EM byte
        let _ = file_as_bytes.remove(file_as_bytes.len().saturating_sub(1));
        // Dropping the second to last byte, ESC byte
        let _ = file_as_bytes.remove(file_as_bytes.len().saturating_sub(1));
        // The first byte of data is the xff version and needs to be dropped too!
        // For prosperity: I noticed this by looking at the bytes of logging_test_name.xff!
        // Like a real nerd!
        let _ = byte_data.remove(0);
        // dropping the second byte, ESC byte
        let _ = byte_data.remove(0);
        // appending data to file, this should move the data instead of copying it into the vector.
        // I hope at least.
        file_as_bytes.extend(byte_data);
        std::fs::write(path, file_as_bytes)?;
        Ok(())
    }
    
}

/// Takes a Vec of logs and returns an xff encoded byte vector
///
/// The xff version used is always the latest version
///
/// # Arguments
/// * `data` - The data to encode
pub fn logs_to_bytes(data: &Vec<Log>) -> Result<Vec<u8>, NabuError> {
    let tokens = logs_tokenizer(data)?;
    // returns a vec of bytes
    serialize_xff(tokens, XFF_VERSION)
}

/// Takes a Vec of logs and returns a Vec of XffValues
///
/// # Arguments
/// * `data` - The data to encode
pub fn logs_tokenizer(data: &Vec<Log>) -> Result<Vec<XffValue>, NabuError> {
    let mut out: Vec<XffValue> = Default::default();
    let cmd1 = XffValue::CommandCharacter(CommandCharacter::FileSeparator);
    let cmd2 = XffValue::CommandCharacter(CommandCharacter::GroupSeparator);
    let cmd3 = XffValue::CommandCharacter(CommandCharacter::RecordSeparator);
    let cmd4 = XffValue::CommandCharacter(CommandCharacter::UnitSeparator);
    for log in data {
        out.push(cmd1.clone());
        for data in log.log_data.clone() {
            out.push(cmd2.clone());
            out.push(XffValue::String(data.name));
            out.push(data.value);
            out.push(cmd3.clone());
            if !data.optional_metadata.is_empty() {
                for (key, value) in data.optional_metadata {
                    out.push(cmd4.clone());
                    out.push(XffValue::String(key));
                    out.push(XffValue::String(value));
                    out.push(cmd4.clone());
                }
            } else {
                out.push(cmd4.clone());
                out.push(cmd4.clone());
            }
            out.push(cmd3.clone());
            out.push(cmd2.clone());
        }
        out.push(cmd1.clone());
    }
    Ok(out)
}

fn decode_log_data(data: &mut Vec<XffValue>, mut value_pos: usize) -> Result<(LogData, usize), NabuError> {
    let mut name: XffValue = Default::default();
    let mut value: XffValue = Default::default();
    let mut optional_metadata: BTreeMap<String, String> = Default::default();
    while data[0] != XffValue::CommandCharacter(CommandCharacter::GroupSeparator) {
        //println!("NAME: {:?}", data[0]);
        name = data.remove(0);
        value_pos += 1;
        //println!("VALUE: {:?}", data[0]);
        value = data.remove(0);
        value_pos += 1;
        optional_metadata = {
            let mut out: BTreeMap<String, String> = BTreeMap::new();
            if data[0] == XffValue::CommandCharacter(CommandCharacter::RecordSeparator) {
                //println!("RS: {:?}", data[0]);
                let _ = data.remove(0);
                value_pos += 1;
                while data[0] != XffValue::CommandCharacter(CommandCharacter::RecordSeparator) {
                    if data[0] == XffValue::CommandCharacter(CommandCharacter::UnitSeparator) {
                        //println!("US: {:?}", data[0]);
                        let _ = data.remove(0);
                        value_pos += 1;
                        if data[0] == XffValue::CommandCharacter(CommandCharacter::UnitSeparator) {
                            //println!("US: {:?}", data[0]);
                            let _ = data.remove(0);
                            value_pos += 1;
                            break;
                        }
                        //println!("KEY: {:?}", data[0]);
                        let key = data.remove(0).as_string();
                        value_pos += 1;
                        //println!("VALUE: {:?}", data[0]);
                        let value = data.remove(0).as_string();
                        value_pos += 1;
                        if key.is_none() || value.is_none() {
                            Err(NabuError::InvalidXFFExtension(format!("Invalid XFF Extension (LoggingWizard), expected a String key-value pair at position {} got KEY: {:?}, VALUE {:?}", value_pos, key, value)))?
                        } else {
                            out.insert(key.unwrap(), value.unwrap());
                            //println!("US: {:?}", data[0]);
                            let exit_marker = data.remove(0);
                            value_pos += 1;
                            if exit_marker != XffValue::CommandCharacter(CommandCharacter::UnitSeparator) {
                                Err(NabuError::InvalidXFFExtension(format!("Invalid XFF Extension (expected LoggingWizard), expected UnitSeparator at value position {} got {:?}", value_pos, exit_marker)))?
                            }
                        }
                    } else {
                        Err(NabuError::InvalidXFFExtension(format!("Invalid XFF, expected UnitSeparator at value position {} got {:?}", value_pos, data[0])))?
                    }
                }
                // remove the trailing RecordSeparator
                //println!("RS: {:?}", data[0]);
                let trailing = data.remove(0);
                value_pos += 1;
                if trailing != XffValue::CommandCharacter(CommandCharacter::RecordSeparator) {
                    Err(NabuError::InvalidXFFExtension(format!("Invalid XFF Extension (expected LoggingWizard), expected RecordSeparator at value position {} got {:?}", value_pos, trailing)))?
                }
                out
            } else {
                BTreeMap::new()
            }
        };
        if data[0] != XffValue::CommandCharacter(CommandCharacter::GroupSeparator) {
            Err(NabuError::InvalidXFFExtension(format!("Invalid XFF Extension (expected LoggingWizard), expected GroupSeparator at value position {} got {:?}", value_pos, data[0])))?
        } else {
            //println!("GS21: {:?}", data[0]);
            let _ = data.remove(0);
            value_pos += 1;
            break;
        }
    }
    return Ok((LogData {
            name: name.as_string().unwrap(),
            value,
            optional_metadata
            }, value_pos));
}

/// Reads a LoggingWizard from a xff file
///
/// # Arguments
/// * `path` - The path to the file
pub fn read_log_wizard<P>(path: P, append: bool) -> Result<LoggingWizard, NabuError> where P: AsRef<std::path::Path> {
    // first byte is version
    let mut value_pos: usize = 2;
    let mut data = read(path.as_ref())?;
    let mut logs: Vec<Log> = Vec::new();
    //println!("START DATA: {:?}", data);
    while data.len() > 0 {
        //println!("FS: {:?}", data[0]);
        match data[0] {
            XffValue::CommandCharacter(CommandCharacter::FileSeparator) => {
                let _ = data.remove(0);
                value_pos += 1;
                // Logdata starts here
                let mut log_data: Vec<LogData> = Vec::new();
                let next_entry: XffValue = data.remove(0);
                //println!("GS: {:?}", next_entry);
                value_pos += 1;
                match next_entry {
                    XffValue::CommandCharacter(CommandCharacter::GroupSeparator) => {
                        let (log, new_byte_pos) = decode_log_data(&mut data, value_pos)?;
                        log_data.push(log);
                        value_pos = new_byte_pos;
                        if data[0] != XffValue::CommandCharacter(CommandCharacter::FileSeparator) {
                            if data[0] == XffValue::CommandCharacter(CommandCharacter::GroupSeparator) {
                                let (log, new_byte_pos) = decode_log_data(&mut data, value_pos)?;
                                log_data.push(log);
                                value_pos = new_byte_pos;
                            } else {
                                Err(NabuError::InvalidXFFExtension(format!("Invalid XFF Extension (expected LoggingWizard), expected FileSeparator at value position {} got {:?}", value_pos, data[0])))?
                            }
                        } else {
                            logs.push(Log { log_data_len: log_data.len(), log_data });
                            //println!("GS2: {:?}", data[0]);
                            let _ = data.remove(0);
                            value_pos += 1;
                            break;
                        }
                    }
                    _ => {
                        Err(NabuError::InvalidXFFExtension(format!("Invalid XFF, expected GroupSeparator at value position {} got {:?}", value_pos, next_entry)))?
                    }
                }
            }
            _ => {
                Err(NabuError::InvalidXFFExtension(format!("Invalid XFF Extension (expected LoggingWizard), expected FileSeparator dodododo at value position {} got {:?}", value_pos, data[0])))?
            }
        }
    }
    Ok(LoggingWizard { logs_len: logs.len(), logs, append, path: path.as_ref().to_path_buf()})
}
