use std::{collections::BTreeMap, path::Path};

use crate::{error::NabuError, key_value_core::new_core_store, serde::{read, write}, xff::value::XffValue};

pub fn read_core(path: &Path) -> Result<BTreeMap<String, XffValue>, NabuError> {
    let content = read(path)?;
    let mut out = new_core_store();
    let mut key: String = Default::default();
    for (index, entry) in content.iter().enumerate() {
        if index.saturating_add(1) % 2 == 0 {
            if key.len() == 0 {
                return Err(NabuError::InvalidXFF(format!("Expected String longer than 0, got {:?}", entry)));
            } else {
                out.insert(key.clone(), entry.clone());
            }
        } else {
            key = {
                match entry {
                    XffValue::String(s) => s.to_string(),
                    _ => return Err(NabuError::InvalidXFF(format!("Expected String for key, got {:?}", entry))),
                }
            };
        }
    }

    Ok(out)
}

pub fn write_core(path: &Path, data: BTreeMap<String, XffValue>) -> Result<(), NabuError> {
    let mut out: Vec<XffValue> = Default::default();
    for (key, value) in data.iter() {
        out.push(XffValue::String(key.to_string()));
        out.push(value.clone());
    }
    write(path, out)
}
