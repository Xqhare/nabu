use std::{collections::BTreeMap, path::PathBuf};

use crate::{error::NabuError, xff::value::XffValue, key_value_core::{read, write}};

#[derive(Debug)]
/// A simple key-value store for storing XFF values
/// Create a new `NabuDB` by using the `NabuDB::new` method
///
/// Note that only XFF files written by the `save` function are supported
///
/// A new `NabuDB` can be created with the `new_nabudb` function or with the `NabuDB::new` method.
///
/// Using the `new_nabudb` function is recommended as it will handle everything for you.
/// This is shown in the example section below.
///
/// Using the `NabuDB::new` method requires you to handle more things, namely making sure the file
/// has a .xff extension and providing a valid `Path` instead of an `&str`.
/// ```rust
/// use std::path::Path;
/// use nabu::features::key_value::store::NabuDB;
///
/// let path = Path::new("xff-example-data/nabuDB_main_example.xff");
/// let mut db = NabuDB::new(path.to_path_buf());
/// assert!(db.is_ok());
/// ```
///
/// # Example
/// ```rust
/// use std::collections::BTreeMap;
/// use nabu::key_value_store::new_nabudb;
/// use nabu::xff::value::{XffValue, CommandCharacter, Data, Number};
///
/// let path = "xff-example-data/nabuDB_main_example.xff";
/// let mut db = new_nabudb(path).unwrap();
///
/// db.insert("key0".to_string(), XffValue::String("value0".to_string()));
/// db.insert("key1".to_string(), XffValue::Number(Number::from(-42)));
/// db.insert("key2".to_string(), XffValue::CommandCharacter(CommandCharacter::LineFeed));
/// db.insert("key3".to_string(), XffValue::Data(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])));
///
/// db.save();
/// 
/// let read = new_nabudb(path).unwrap();
/// assert_eq!(read.get("key0").unwrap(), db.get("key0").unwrap());
/// assert_eq!(read.get("key1").unwrap(), db.get("key1").unwrap());
/// assert_eq!(read.get("key2").unwrap(), db.get("key2").unwrap());
/// assert_eq!(read.get("key3").unwrap(), db.get("key3").unwrap());
///
/// db.set_auto_save(true);
/// db.insert("key4".to_string(), XffValue::String("value4".to_string()));
/// let read = new_nabudb(path).unwrap();
/// assert_eq!(db.get("key4").unwrap(), read.get("key4").unwrap());
///
/// if db.contains_key("key4") {
///     let _ = db.remove("key4");
/// }
///
/// println!("All keys:");
/// for key in db.keys() {
///     println!("{}", key);
/// }
///
/// let map: BTreeMap<String, XffValue> = db.to_map();
///
/// let get_key_0 = db.get("key0");
/// assert_eq!(get_key_0.unwrap(), &XffValue::String("value0".to_string()));
///
/// let (key, value) = db.get_key_value("key0").unwrap();
/// assert_eq!(key, &"key0".to_string());
/// assert_eq!(value, &XffValue::String("value0".to_string()));
///
/// println!("All key-values:");
/// for (key, value) in db.iter() {
///     println!("{}: {:?}", key, value);
/// }
///
/// let len = db.len();
///
/// assert_eq!(db.get("key0").unwrap(), &XffValue::String("value0".to_string()));
/// assert_eq!(db.get("key1").unwrap(), &XffValue::Number(Number::from(-42)));
/// assert_eq!(db.get("key2").unwrap(), &XffValue::CommandCharacter(CommandCharacter::LineFeed));
/// assert_eq!(db.get("key3").unwrap(), &XffValue::Data(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])));
/// assert_eq!(len, 4);
///
/// db.clear();
/// let read = new_nabudb(path).unwrap();
/// assert!(read.len() == 0);
/// ```
pub struct NabuDB {
    core: BTreeMap<String, XffValue>,
    path: std::path::PathBuf,
    length: usize,
    auto_save: bool,
}

impl NabuDB {
    /// Creates a new `NabuDB` from a path
    /// If the path does not exist, a new `NabuDB` will be created,
    /// otherwise the `NabuDB` will be loaded from the file
    ///
    /// Note that only XFF files written by the `save` function are supported
    ///
    /// # Arguments
    /// * `path` - The path to the file to load
    ///
    /// # Example
    /// ```rust
    /// use nabu::key_value_store::new_nabudb;
    ///
    /// let db = new_nabudb("xff-example-data/nabuDB.xff");
    /// assert!(db.is_ok());
    /// ```
    ///
    /// # Errors
    /// Returns IO errors when issues with reading the file from disk occur
    /// Also returns Nabu errors when issues with the XFF format occur
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

    /// Saves the `NabuDB` to disk
    ///
    /// # Example
    /// ```rust
    /// use nabu::key_value_store::new_nabudb;
    /// use nabu::xff::value::{XffValue, CommandCharacter, Data, Number};
    /// 
    /// let mut db = new_nabudb("xff-example-data/nabuDB.xff").unwrap();
    /// db.insert("key0".to_string(), XffValue::String("value0".to_string()));
    /// db.insert("key1".to_string(), XffValue::Number(Number::from(-42)));
    /// db.insert("key2".to_string(), XffValue::CommandCharacter(CommandCharacter::LineFeed));
    /// db.insert("key3".to_string(), XffValue::Data(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])));
    /// db.save();
    /// ```
    pub fn save(&mut self) -> Result<(), NabuError> {
        write(&self.path, self.core.clone())?;
        Ok(())
    }

    /// Enables or disables the auto-save feature
    /// By default, the auto-save feature is off
    ///
    /// Please note that this feature will save the entire `NabuDB` on every change and can be
    /// quite impactful for large `NabuDB`s
    ///
    /// # Arguments
    /// * `auto_save` - boolean to set the auto-save feature to on or off
    ///
    /// # Example
    /// ```rust
    /// use nabu::key_value_store::new_nabudb;
    ///
    /// let mut db = new_nabudb("xff-example-data/nabuDB.xff").unwrap();
    /// db.set_auto_save(true);
    /// ```
    pub fn set_auto_save(&mut self, auto_save: bool) {
        self.auto_save = auto_save;
    }

    /// Helper function to call everywhere to save the `NabuDB`
    /// Checks the need to save internally and calls `save` if needed
    fn auto_save(&mut self) -> Result<(), NabuError> {
        if self.auto_save {
            self.save()
        } else {
            Ok(())
        }
    }

    /// Clears all entries in the `NabuDB`
    /// Also saves the `NabuDB` if the `auto_save` feature is enabled
    ///
    /// Please take care when using this function, as it will clear all entries in the `NabuDB` - Loss of data is the feature!
    ///
    /// # Example
    /// ```rust
    /// use nabu::key_value_store::new_nabudb;
    /// use nabu::xff::value::XffValue;
    ///
    /// let mut db = new_nabudb("xff-example-data/nabuDB_clear.xff").unwrap();
    /// db.insert("key0".to_string(), XffValue::String("value0".to_string()));
    /// db.clear();
    ///
    /// assert_eq!(db.len(), 0);
    /// ```
    pub fn clear(&mut self)  -> Result<(), NabuError> {
        self.core.clear();
        self.length = 0;
        let _ = self.auto_save();
        Ok(())
    }

    /// Checks if the `NabuDB` contains an entry with the given key
    ///
    /// # Arguments
    /// * `key` - The key to check for
    ///
    /// # Example
    /// ```rust
    /// use nabu::key_value_store::new_nabudb;
    /// use nabu::xff::value::XffValue;
    ///
    /// let mut db = new_nabudb("xff-example-data/nabuDB_contains.xff").unwrap();
    /// db.insert("key0".to_string(), XffValue::String("value0".to_string()));
    ///
    /// assert!(db.contains_key("key0"));
    /// assert!(!db.contains_key("key1"));
    /// ```
    pub fn contains_key(&self, key: &str) -> bool {
        self.core.contains_key(key)
    }

    /// Returns a list of all keys in the `NabuDB`
    ///
    /// # Example
    /// ```rust
    /// use nabu::key_value_store::new_nabudb;
    /// use nabu::xff::value::XffValue;
    ///
    /// let mut db = new_nabudb("xff-example-data/nabuDB_keys.xff").unwrap();
    /// db.insert("key0".to_string(), XffValue::String("value0".to_string()));
    /// db.insert("key1".to_string(), XffValue::String("value1".to_string()));
    ///
    /// assert_eq!(db.keys().len(), 2);
    /// ```
    pub fn keys(&self) -> Vec<String> {
        self.core.keys().cloned().collect()
    }

    /// Inserts an entry into the `NabuDB`
    /// Also saves the `NabuDB` if the `auto_save` feature is enabled
    ///
    /// # Arguments
    /// * `key` - The key to insert
    /// * `value` - The value to insert
    ///
    /// # Example
    /// ```rust
    /// use nabu::key_value_store::new_nabudb;
    /// use nabu::xff::value::XffValue;
    ///
    /// let mut db = new_nabudb("xff-example-data/nabuDB_insert.xff").unwrap();
    /// db.insert("key0".to_string(), XffValue::String("value0".to_string()));
    ///
    /// assert_eq!(db.len(), 1);
    /// ```
    pub fn insert(&mut self, key: String, value: XffValue) {
        self.core.insert(key, value);
        self.length = self.core.len();
        let _ = self.auto_save();
    }

    /// Removes an entry from the `NabuDB`
    /// Also saves the `NabuDB` if the `auto_save` feature is enabled
    ///
    /// # Arguments
    /// * `key` - The key to remove
    ///
    /// # Example
    /// ```rust
    /// use nabu::key_value_store::new_nabudb;
    /// use nabu::xff::value::XffValue;
    ///
    /// let mut db = new_nabudb("xff-example-data/nabuDB_remove.xff").unwrap();
    /// db.insert("key0".to_string(), XffValue::String("value0".to_string()));
    /// db.remove("key0");
    ///
    /// assert_eq!(db.len(), 0);
    /// ```
    pub fn remove(&mut self, key: &str) -> Option<XffValue> {
        let out = self.core.remove(key);
        self.length -= 1;
        let _ = self.auto_save();
        out
    }

    /// Returns the value of the `NabuDB` as a `BTreeMap`
    ///
    /// # Example
    /// ```rust
    /// use nabu::key_value_store::new_nabudb;
    /// use nabu::xff::value::XffValue;
    ///
    /// let mut db = new_nabudb("xff-example-data/nabuDB_map_example.xff").unwrap();
    /// db.insert("key0".to_string(), XffValue::String("value0".to_string()));
    /// let map = db.to_map();
    /// assert_eq!(map.len(), 1);
    /// ```
    pub fn to_map(&self) -> BTreeMap<String, XffValue> {
        self.core.clone()
    }

    /// Returns a reference to the value of the `NabuDB` at the given key
    /// Returns `None` if the key does not exist
    ///
    /// # Arguments
    /// * `key` - The key to get
    ///
    /// # Example
    /// ```rust
    /// use nabu::key_value_store::new_nabudb;
    /// use nabu::xff::value::XffValue;
    ///
    /// let mut db = new_nabudb("xff-example-data/nabuDB.xff").unwrap();
    /// db.insert("key0".to_string(), XffValue::String("value0".to_string()));
    /// db.insert("key1".to_string(), XffValue::String("value1".to_string()));
    ///
    /// let key0 = db.get("key0");
    /// let key1 = db.get("key1");
    ///
    /// assert!(key0.is_some());
    /// assert!(key1.is_some());
    /// assert_eq!(key0.unwrap(), &XffValue::String("value0".to_string()));
    /// ```
    pub fn get(&self, key: &str) -> Option<&XffValue> {
        self.core.get(key)
    }

    /// Returns the key-value pair of the `NabuDB` at the given key
    ///
    /// # Arguments
    /// * `key` - The key to get
    ///
    /// # Example
    /// ```rust
    /// use nabu::key_value_store::new_nabudb;
    /// use nabu::xff::value::XffValue;
    ///
    /// let mut db = new_nabudb("xff-example-data/nabuDB.xff").unwrap();
    /// db.insert("key0".to_string(), XffValue::String("value0".to_string()));
    ///
    /// let key0 = db.get_key_value("key0");
    ///
    /// assert!(key0.is_some());
    /// assert_eq!(key0.unwrap(), (&"key0".to_string(), &XffValue::String("value0".to_string())));
    /// ```
    pub fn get_key_value(&self, key: &str) -> Option<(&String, &XffValue)> {
        self.core.get_key_value(key)
    }

    /// Returns an iterator over the key-value pairs of the `NabuDB`
    ///
    /// # Example
    /// ```rust
    /// use nabu::key_value_store::new_nabudb;
    /// use nabu::xff::value::XffValue;
    ///
    /// let mut db = new_nabudb("xff-example-data/nabuDB.xff").unwrap();
    /// db.insert("key0".to_string(), XffValue::String("value0".to_string()));
    /// db.insert("key1".to_string(), XffValue::String("value1".to_string()));
    ///
    /// let iter = db.iter();
    /// for (key, value) in iter {
    ///     println!("{}: {:?}", key, value);
    /// }
    /// ```
    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, String, XffValue> {
        self.core.iter()
    }

    /// Returns the number of key-value pairs in the `NabuDB`
    ///
    /// # Example
    /// ```rust
    /// use nabu::key_value_store::new_nabudb;
    /// use nabu::xff::value::XffValue;
    ///
    /// let mut db = new_nabudb("xff-example-data/nabuDB_len.xff").unwrap();
    ///
    /// assert_eq!(db.len(), 0);
    /// db.insert("key0".to_string(), XffValue::String("value0".to_string()));
    /// db.insert("key1".to_string(), XffValue::String("value1".to_string()));
    /// assert_eq!(db.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.length
    }
}
