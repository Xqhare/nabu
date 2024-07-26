use std::{collections::BTreeMap, path::PathBuf};

use crate::{error::NabuError, xff::value::XffValue, key_value_core::{read, write}};

#[derive(Debug)]
pub struct NabuDB {
    core: BTreeMap<String, XffValue>,
    path: std::path::PathBuf,
    length: usize,
    auto_save: bool,
}

impl NabuDB {
    pub fn new(path: PathBuf) -> Result<Self, NabuError> {
        if path.exists() {
            let core: BTreeMap<String, XffValue> = read(path.as_path())?;
            Ok(NabuDB {
                length: core.len(),
                core,
                path: path.to_path_buf(),
                auto_save: false,
            })
        } else {
            Ok(NabuDB {
                length: 0,
                core: BTreeMap::new(),
                path,
                auto_save: false,
            })
        }
    }

    pub fn save(&mut self) -> Result<(), NabuError> {
        write(&self.path, self.core.clone())?;
        Ok(())
    }

    pub fn set_auto_save(&mut self, auto_save: bool) {
        self.auto_save = auto_save;
    }

    fn auto_save(&mut self) -> Result<(), NabuError> {
        if self.auto_save {
            self.save()
        } else {
            Ok(())
        }
    }

    pub fn clear(&mut self)  -> Result<(), NabuError> {
        self.core.clear();
        self.length = 0;
        let _ = self.auto_save();
        Ok(())
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.core.contains_key(key)
    }

    pub fn insert(&mut self, key: String, value: XffValue) {
        self.core.insert(key, value);
        self.length += 1;
        let _ = self.auto_save();
    }

    pub fn remove(&mut self, key: String) -> Option<XffValue> {
        let out = self.core.remove(&key);
        self.length -= 1;
        let _ = self.auto_save();
        out
    }

    pub fn get(&self, key: &str) -> Option<&XffValue> {
        self.core.get(key)
    }

    pub fn get_key_value(&self, key: &str) -> Option<(&String, &XffValue)> {
        self.core.get_key_value(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut XffValue> {
        self.core.get_mut(key)
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, String, XffValue> {
        self.core.iter()
    }

    pub fn len(&self) -> usize {
        self.length
    }
}
