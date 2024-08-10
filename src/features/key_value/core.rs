use std::{collections::BTreeMap, path::Path};

use crate::{
    error::NabuError,
    key_value_core::new_core_store,
    serde::{read, write},
    xff::value::XffValue,
};

/// Reads the content of a XFF file and returns a BTreeMap
/// Please note that only XFF files written by the `write_core` function are supported
///
/// # Arguments
/// * `path` - The path to the file to read
///
/// # Example
/// ```rust
/// use std::path::Path;
/// use nabu::features::key_value::core::read_core;
///
/// let path = Path::new("xff-example-data/key_value_core.xff");
/// let data = read_core(path);
/// assert!(data.is_ok());
/// ```
pub fn read_core(path: &Path) -> Result<BTreeMap<String, XffValue>, NabuError> {
    let content = read(path)?;
    let mut out = new_core_store();
    let mut key: String = Default::default();
    for (index, entry) in content.iter().enumerate() {
        if index.saturating_add(1) % 2 == 0 {
            if key.len() == 0 {
                return Err(NabuError::InvalidXFFExtension("key value core".to_string(), format!(
                    "Expected String longer than 0, got {:?}",
                    entry
                )));
            } else {
                out.insert(key.clone(), entry.clone());
            }
        } else {
            key = {
                match entry {
                    XffValue::String(s) => s.to_string(),
                    _ => {
                        return Err(NabuError::InvalidXFFExtension("key value core".to_string(), format!(
                            "Expected String for key, got {:?}",
                            entry
                        )))
                    }
                }
            };
        }
    }
    Ok(out)
}

/// Writes a BTreeMap to a XFF file
///
/// # Arguments
/// * `path` - The path to the file to write
/// * `data` - The BTreeMap to write
///
/// # Example
/// ```rust
/// use std::path::Path;
/// use std::collections::BTreeMap;
/// use nabu::features::key_value::core::write_core;
/// use nabu::xff::value::{XffValue, Number};
///
/// let mut data = BTreeMap::new();
/// data.insert("key0".to_string(), XffValue::String("value0".to_string()));
/// data.insert("key1".to_string(), XffValue::Number(Number::from(42)));
///
/// let path = Path::new("xff-example-data/key_value_core.xff");
/// let write = write_core(path, data);
/// assert!(write.is_ok());
/// ```
pub fn write_core(path: &Path, data: BTreeMap<String, XffValue>) -> Result<(), NabuError> {
    let mut out: Vec<XffValue> = Default::default();
    for (key, value) in data.iter() {
        out.push(XffValue::String(key.to_string()));
        out.push(value.clone());
    }
    write(path, out)
}
