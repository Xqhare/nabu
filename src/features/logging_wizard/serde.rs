use std::collections::BTreeMap;
use crate::{serde::{read, write}, xff::value::{XffValue, CommandCharacter}, error::NabuError, features::logging_wizard::{Log, LogData}};

use super::LoggingWizard;

impl LoggingWizard {

    pub fn write_log_wizard(&self) -> Result<(), NabuError> {
        todo!()
    }

    fn append(&self) -> Result<(), NabuError> {
        let mut file_as_bytes = std::fs::read(self.path.clone())?;
        // Dropping the last byte, EM byte
        let _ = file_as_bytes[file_as_bytes.len() - 1];
        let data = self.to_bytes();
        // appending data to file
        file_as_bytes.extend(data);
        std::fs::write(self.path.clone(), file_as_bytes)?;
        Ok(())
    }

    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

pub fn read_log_wizard<P>(path: P) -> Result<LoggingWizard, NabuError> where P: AsRef<std::path::Path> {
    let mut data = read(path.as_ref())?;
    let mut logs: Vec<Log> = Vec::new();
    while data.len() > 0 {
        let current_entry: XffValue = data.remove(0);
        match current_entry {
            XffValue::CommandCharacter(CommandCharacter::FileSeparator) => {
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
                                                Err(NabuError::InvalidXFF(format!("Invalid XFF, expected a String key-value pair got KEY: {:?}, VALUE {:?}", key, value)))?
                                            } else {
                                                out.insert(key.unwrap(), value.unwrap());
                                                let exit_marker = data.remove(0);
                                                if exit_marker != XffValue::CommandCharacter(CommandCharacter::UnitSeparator) {
                                                    Err(NabuError::InvalidXFF(format!("Invalid XFF, expected UnitSeparator got {:?}", exit_marker)))?
                                                }
                                            }
                                        } else {
                                            Err(NabuError::InvalidXFF(format!("Invalid XFF, expected UnitSeparator got {:?}", data[0])))?
                                        }
                                    }
                                    // remove the trailing RecordSeparator
                                    let trailing = data.remove(0);
                                    if trailing != XffValue::CommandCharacter(CommandCharacter::RecordSeparator) {
                                        Err(NabuError::InvalidXFF(format!("Invalid XFF, expected RecordSeparator got {:?}", trailing)))?
                                    }
                                    out
                                } else {
                                    BTreeMap::new()
                                }
                            };
                            if data[0] != XffValue::CommandCharacter(CommandCharacter::GroupSeparator) {
                                Err(NabuError::InvalidXFF(format!("Invalid XFF, expected GroupSeparator got {:?}", data[0])))?
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
                            Err(NabuError::InvalidXFF(format!("Invalid XFF, expected FileSeparator got {:?}", data[0])))?
                        } else {
                            logs.push(Log { log_data });
                            let _ = data.remove(0);
                        }
                    }
                    _ => {
                        Err(NabuError::InvalidXFF(format!("Invalid XFF, expected GroupSeparator got {:?}", next_entry)))?
                    }
                }
            }
            _ => {
                Err(NabuError::InvalidXFF(format!("Invalid XFF, expected FileSeparator got {:?}", current_entry)))?
            }
        }
    }
    Ok(LoggingWizard { logs, path: path.as_ref().to_path_buf() })
}
