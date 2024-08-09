use crate::{
    error::NabuError,
    features::logging_wizard::{Log, LogData, LoggingWizard},
    serde::read,
    xff::{
        serializer::{serialize_xff, write_bytes_to_file},
        value::{CommandCharacter, XffValue},
    },
    XFF_VERSION,
};
use std::{collections::{BTreeMap, VecDeque}, path::Path};

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
fn logs_to_bytes(data: &Vec<Log>) -> Result<Vec<u8>, NabuError> {
    let tokens = logs_tokenizer(data)?;
    // returns a vec of bytes
    serialize_xff(tokens, XFF_VERSION)
}

/// Takes a Vec of logs and returns a Vec of XffValues
///
/// # Arguments
/// * `data` - The data to encode
fn logs_tokenizer(data: &Vec<Log>) -> Result<Vec<XffValue>, NabuError> {
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

pub fn read_log_wizard<P>(path: P, append: bool) -> Result<LoggingWizard, NabuError>
where
    P: AsRef<std::path::Path>,
{
    let mut value_pos: usize = 1;
    // creating the Token array
    let mut data: VecDeque<XffValue> = read(path.as_ref())?.into_iter().collect();
    let mut logs: Vec<Log> = Vec::new();
    while data.len() > 0 {
        match data[0] {
            XffValue::CommandCharacter(CommandCharacter::FileSeparator) => {
                // remove the FileSeparator
                let _ = data.pop_front();
                value_pos += 1;
                // build log
                let log = decode_log(&mut data, &mut value_pos)?;
                logs.push(log);
                if data[0] == XffValue::CommandCharacter(CommandCharacter::FileSeparator) {
                    // drop trailing FileSeparator
                    let _ = data.pop_front();
                    value_pos += 1;
                    if data.len() == 0 {
                        break;
                    } else {
                        continue;
                    }
                }
            }
            _ => Err(NabuError::InvalidXFFExtension(format!(
                "Invalid XFF Extension (expected LoggingWizard), expected FileSeparator at value position {} got {:?}",
                value_pos, data[0]
            )))?
        }
    }
    Ok(LoggingWizard {
        logs_len: logs.len(),
        logs,
        append,
        path: path.as_ref().to_path_buf().with_extension("xff"),
    })
}

fn decode_log(data: &mut VecDeque<XffValue>, value_pos: &mut usize) -> Result<Log, NabuError> {
    let mut log_data: Vec<LogData> = Vec::new();
    while data.len() > 0 {
        match data[0] {
            XffValue::CommandCharacter(CommandCharacter::GroupSeparator) => {
                // remove the GroupSeparator
                let _ = data.pop_front();
                *value_pos += 1;
                // build log data
                log_data.push(decode_log_data(data, value_pos)?);
                if data[0] == XffValue::CommandCharacter(CommandCharacter::GroupSeparator) {
                    // drop trailing GroupSeparator
                    let _ = data.pop_front();
                    *value_pos += 1;
                    match data[0] {
                        XffValue::CommandCharacter(CommandCharacter::GroupSeparator) => {
                            continue;
                        },
                        XffValue::CommandCharacter(CommandCharacter::FileSeparator) => {
                            // build log
                            return Ok(Log::from(log_data));
                        },
                        _ => Err(NabuError::InvalidXFFExtension(format!(
                            "Invalid XFF, expected FileSeparator or GroupSeparator at value position {} got {:?}",
                            value_pos, data[0]
                        )))?
                    }
                }
            }
            _ => Err(NabuError::InvalidXFFExtension(format!(
                "Invalid XFF, expected GroupSeparator at value position {} got {:?}",
                value_pos, data[0]
            )))?,
        }
    }
    // should never get here, so error
    Err(NabuError::InvalidXFFExtension(format!(
        "Invalid XFF, expected GroupSeparator at value position {} got End of file!",
        value_pos
    )))
}

fn decode_log_data(data: &mut VecDeque<XffValue>, value_pos: &mut usize) -> Result<LogData, NabuError> {
    // I was paniking with remove(0) implicitly anyway
    let name = data.pop_front().unwrap().as_string();
    //println!("name: {:?}", name);
    *value_pos += 1;
    let value = data.pop_front().unwrap();
    //println!("value: {:?}", value);
    *value_pos += 1;
    let mut optional_metadata: BTreeMap<String, String> = BTreeMap::new();

    match data[0] {
        XffValue::CommandCharacter(CommandCharacter::RecordSeparator) => {
            // remove the RecordSeparator
            let _ = data.pop_front();
            *value_pos += 1;
            for metadata in decode_metadata(data, value_pos)? {
                optional_metadata.insert(metadata.0, metadata.1);
            }
            if data[0] == XffValue::CommandCharacter(CommandCharacter::RecordSeparator) {
                // drop trailing RecordSeparator
                let _ = data.pop_front();
                *value_pos += 1;
                return Ok(LogData {
                    name: name.unwrap(),
                    value,
                    optional_metadata,
                });
            } else {
                Err(NabuError::InvalidXFFExtension(format!(
                    "Invalid XFF, expected RecordSeparator at value position {} got {:?}",
                    value_pos, data[0]
                )))?
            }
        }
        _ => Err(NabuError::InvalidXFFExtension(format!(
            "Invalid XFF, expected RecordSeparator at value position {} got {:?} dododo ",
            value_pos, data[0]
        )))?,
    }
}

fn decode_metadata(
    data: &mut VecDeque<XffValue>,
    value_pos: &mut usize,
) -> Result<Vec<(String, String)>, NabuError> {
    let mut out: Vec<(String, String)> = Vec::new();
    // sanity match
    match data[0] {
        XffValue::CommandCharacter(CommandCharacter::UnitSeparator) => {
            // actual loop
            while data.len() > 0 {
                match data[0] {
                    XffValue::CommandCharacter(CommandCharacter::UnitSeparator) => {
                        // remove the UnitSeparator
                        let _ = data.pop_front();
                        *value_pos += 1;
                        // no metadata, so just end
                        if data[0] == XffValue::CommandCharacter(CommandCharacter::UnitSeparator) {
                            // drop trailing UnitSeparator
                            let _ = data.pop_front();
                            *value_pos += 1;
                            return Ok(out);
                        } else {
                            // build metadata
                            out.push(decode_metadata_entry(data, value_pos)?);
                        }
                    },
                    XffValue::CommandCharacter(CommandCharacter::RecordSeparator) => {
                        return Ok(out);
                    },
                    _ => {
                        return Err(NabuError::InvalidXFFExtension(format!("Invalid XFF, expected UnitSeparator or RecordSeparator at value position {} got {:?}", value_pos, data[0])))
                    }
                }
            }
            Err(NabuError::InvalidXFFExtension(format!(
                "Invalid XFF, expected RecordSeparator at value position {} got End of file!",
                value_pos
            )))
        }
        // This should never ever happen, decode_metadata is only ever called with data that is a UnitSeparator
        _ => Err(NabuError::InvalidXFFExtension(format!(
            "Invalid XFF, expected UnitSeparator at value position {} got {:?}",
            value_pos, data[0]
        )))?,
    }
}

fn decode_metadata_entry(
    data: &mut VecDeque<XffValue>,
    value_pos: &mut usize,
) -> Result<(String, String), NabuError> {
    let name = data.pop_front().unwrap().as_string();
    //println!("name: {:?}", name);
    *value_pos += 1;
    let value = data.pop_front().unwrap().as_string();
    //println!("value: {:?}", value);
    *value_pos += 1;
    if data[0] == XffValue::CommandCharacter(CommandCharacter::UnitSeparator) {
        // remove the trailing UnitSeparator
        let _ = data.pop_front();
        *value_pos += 1;
        Ok((name.unwrap(), value.unwrap()))
    } else {
        Err(NabuError::InvalidXFFExtension(format!(
            "Invalid XFF, expected UnitSeparator at value position {} got {:?}",
            value_pos, data[0]
        )))
    }
}
