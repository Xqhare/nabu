use std::{collections::BTreeMap, path::Path};
use crate::{XFF_VERSION, serde::{read, write}, xff::{value::{XffValue, CommandCharacter, Data}, serializer::{serialize_xff, write_bytes_to_file}}, error::NabuError, features::logging_wizard::{LoggingWizard, Log, LogData}};

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
    let data = logs_to_bytes(data)?;
    if !path.exists() {
        std::fs::write(path, file_as_bytes)?;
        Ok(())
    } else {
        let mut file_as_bytes: Vec<u8> = std::fs::read(path)?;
        // Dropping the last byte, EM byte
        let _ = file_as_bytes[file_as_bytes.len() - 1];
        // appending data to file, this should move the data instead of copying it into the vector.
        // I hope at least.
        file_as_bytes.extend(data);
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

/// Reads a LoggingWizard from a xff file
///
/// # Arguments
/// * `path` - The path to the file
pub fn read_log_wizard<P>(path: P) -> Result<LoggingWizard, NabuError> where P: AsRef<std::path::Path> {
    let mut data = read(path.as_ref())?;
    let mut logs: Vec<Log> = Vec::new();
    while data.len() > 0 {
        match data[0] {
            XffValue::CommandCharacter(CommandCharacter::FileSeparator) => {
                let _ = data.remove(0);
                // Logdata starts here
                let mut log_data: Vec<LogData> = Vec::new();
                let next_entry: XffValue = data.remove(0);
                match next_entry {
                    XffValue::CommandCharacter(CommandCharacter::GroupSeparator) => {
                        while data[0] != XffValue::CommandCharacter(CommandCharacter::GroupSeparator) {
                            let name = data.remove(0);
                            let value = data.remove(0);
                            let optional_metadata: BTreeMap<String, String> = {
                                let mut out: BTreeMap<String, String> = BTreeMap::new();
                                if data[0] == XffValue::CommandCharacter(CommandCharacter::RecordSeparator) {
                                    let _ = data.remove(0);
                                    while data[0] != XffValue::CommandCharacter(CommandCharacter::RecordSeparator) {
                                        if data[0] == XffValue::CommandCharacter(CommandCharacter::UnitSeparator) {
                                            let _ = data.remove(0);
                                            let key = data.remove(0).as_string();
                                            let value = data.remove(0).as_string();
                                            if key.is_none() || value.is_none() {
                                                Err(NabuError::InvalidXFFExtension(format!("Invalid XFF Extension (LoggingWizard), expected a String key-value pair got KEY: {:?}, VALUE {:?}", key, value)))?
                                            } else {
                                                out.insert(key.unwrap(), value.unwrap());
                                                let exit_marker = data.remove(0);
                                                if exit_marker != XffValue::CommandCharacter(CommandCharacter::UnitSeparator) {
                                                    Err(NabuError::InvalidXFFExtension(format!("Invalid XFF Extension (expected LoggingWizard), expected UnitSeparator got {:?}", exit_marker)))?
                                                }
                                            }
                                        } else {
                                            Err(NabuError::InvalidXFFExtension(format!("Invalid XFF, expected UnitSeparator got {:?}", data[0])))?
                                        }
                                    }
                                    // remove the trailing RecordSeparator
                                    let trailing = data.remove(0);
                                    if trailing != XffValue::CommandCharacter(CommandCharacter::RecordSeparator) {
                                        Err(NabuError::InvalidXFFExtension(format!("Invalid XFF Extension (expected LoggingWizard), expected RecordSeparator got {:?}", trailing)))?
                                    }
                                    out
                                } else {
                                    BTreeMap::new()
                                }
                            };
                            if data[0] != XffValue::CommandCharacter(CommandCharacter::GroupSeparator) {
                                Err(NabuError::InvalidXFFExtension(format!("Invalid XFF Extension (expected LoggingWizard), expected GroupSeparator got {:?}", data[0])))?
                            } else {
                                log_data.push(LogData {
                                name: name.as_string().unwrap(),
                                value,
                                optional_metadata
                                });
                                let _ = data.remove(0);
                            }
                        }
                        if data[0] != XffValue::CommandCharacter(CommandCharacter::GroupSeparator) {
                            Err(NabuError::InvalidXFFExtension(format!("Invalid XFF Extension (expected LoggingWizard), expected FileSeparator got {:?}", data[0])))?
                        } else {
                            logs.push(Log { log_data });
                            let _ = data.remove(0);
                        }
                    }
                    _ => {
                        Err(NabuError::InvalidXFFExtension(format!("Invalid XFF, expected GroupSeparator got {:?}", next_entry)))?
                    }
                }
            }
            _ => {
                Err(NabuError::InvalidXFFExtension(format!("Invalid XFF Extension (expected LoggingWizard), expected FileSeparator got {:?}", data[0])))?
            }
        }
    }
    Ok(LoggingWizard { logs, path: path.as_ref().to_path_buf() })
}
