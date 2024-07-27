mod error;

pub mod xff;

const XFF_VERSION: u8 = 0;

/// Module to serialize and deserialize XFF files
/// 
/// # Example
/// ```rust
/// use std::fs::remove_file;
/// use nabu::serde::{read, write};
/// use nabu::xff::value::{CommandCharacter, Data, Number, XffValue};
/// // No matter what the extension of the path you provide, it will be converted to .xff
/// let path = "xff-example-data/v0.txt";
/// let path_2 = "xff-example-data/v0.xff";
/// let data = {
///     vec![
///         XffValue::String("hello mom".to_string()),
///     ]
/// };
/// let tmp = write(path, data.clone());
/// assert!(tmp.is_ok());
/// let tmp_2 = read(path_2);
/// assert!(tmp_2.is_ok());
/// let ok = tmp_2.unwrap();
/// assert_eq!(ok[0], data[0]);
/// // delete file
/// remove_file(path_2).unwrap();
/// ```
pub mod serde {
    use crate::error::NabuError;
    use crate::xff::deserializer::deserialize_xff;
    use crate::xff::serializer::serialize_xff;
    use crate::xff::value::XffValue;
    use crate::XFF_VERSION;

    /// Reads the content of a XFF file and returns a Vec of XffValues
    ///
    /// # Arguments
    /// * `path` - The path to the file to read
    ///
    /// # Error
    /// Errors if the file is not a valid XFF file or if an IO error occurs
    ///
    /// # Example
    /// ```rust
    /// use nabu::serde::read;
    ///
    /// let tmp = read("xff-example-data/v0.xff");
    /// assert!(tmp.is_ok());
    /// ```
    pub fn read<P>(path: P) -> Result<Vec<XffValue>, NabuError> where P: AsRef<std::path::Path> {
        let path_with_xff_extension = path.as_ref().with_extension("xff");
        deserialize_xff(&path_with_xff_extension)
    }

    /// Writes a Vec of XffValues to a XFF file
    ///
    /// # Arguments
    /// * `path` - The path to the file to write
    ///
    /// # Error
    /// Only errors if an IO error occurs
    ///
    /// # Example
    /// ```rust
    /// use nabu::serde::write;
    /// use nabu::xff::value::{CommandCharacter, Data, Number, XffValue};
    ///
    /// let data = {
    ///     vec![
    ///         XffValue::String("hello mom".to_string()),
    ///     ]
    /// };
    /// let tmp = write("xff-example-data/v0.xff", data.clone());
    /// assert!(tmp.is_ok());
    /// ```
    pub fn write<P>(path: P, data: Vec<XffValue>) -> Result<(), NabuError> where P: AsRef<std::path::Path> {
        let path_with_xff_extension = path.as_ref().with_extension("xff");
        serialize_xff(&path_with_xff_extension, data, XFF_VERSION)
    }

    /// Writes a Vec of XffValues to a XFF file with a specific XFF version
    /// Provided for backwards compatibility and convenience
    ///
    /// # Arguments
    /// * `path` - The path to the file to write
    /// * `data` - The data to write
    /// * `xff_version` - The XFF version to use
    ///
    /// # Error
    /// Only errors if an IO error occurs
    ///
    /// # Example
    /// ```rust
    /// use nabu::serde::write_legacy;
    /// use nabu::xff::value::XffValue;
    ///
    /// let data = {
    ///     vec![
    ///         XffValue::String("hello mom".to_string()),
    ///     ]
    /// };
    /// let tmp = write_legacy("xff-example-data/v0.xff", data.clone(), 0);
    /// assert!(tmp.is_ok());
    /// ```
    pub fn write_legacy<P>(path: P, data: Vec<XffValue>, xff_version: u8) -> Result<(), NabuError> where P: AsRef<std::path::Path> {
        let path_with_xff_extension = path.as_ref().with_extension("xff");
        serialize_xff(&path_with_xff_extension, data, xff_version)
    }

    /// A convenience function to remove an XFF file
    /// Deletes a file from disk, take care!
    ///
    /// # Arguments
    /// * `path` - The path to the file to remove
    ///
    /// # Error
    /// Only errors if an IO error occurs
    ///
    /// # Example
    /// ```rust
    /// // To remove a file we need to have one, just ignore
    /// use nabu::serde::write;
    /// use nabu::xff::value::XffValue;
    ///
    /// let data = {
    ///     vec![
    ///         XffValue::String("hello mom".to_string()),
    ///     ]
    /// };
    /// let _ = write("xff-example-data/remove.xff", data.clone());
    ///
    /// // Now we can remove it
    /// use nabu::serde::remove;
    ///
    /// let tmp = remove("xff-example-data/remove.xff");
    /// assert!(tmp.is_ok());
    /// ```
    pub fn remove<P>(path: P) -> Result<(), NabuError> where P: AsRef<std::path::Path> {
        let path_with_xff_extension = path.as_ref().with_extension("xff");
        Ok(std::fs::remove_file(path_with_xff_extension)?)
    }
}

// Remember to add any and all new features to this!
// It's literally a feature-gate for all features - leads to cleaner and leaner code I hope
#[cfg(any(feature = "key_value_core", feature = "key_value_store", feature = "config_wizard", feature = "logging_wizard"))]
pub mod features;

#[cfg(feature = "key_value_core")]
/// Module to read and write a basic key-value store
pub mod key_value_core {
    use std::collections::BTreeMap;

    use crate::{error::NabuError, features::key_value::core::{read_core, write_core}, xff::value::XffValue};

    /// Reads the content of a XFF file and returns a BTreeMap
    /// Please note that only XFF files written by the `write` function of this module are supported
    pub fn read<P>(path: P) -> Result<BTreeMap<String, XffValue>, NabuError> where P: AsRef<std::path::Path>{
        let path_with_xff_extension = path.as_ref().with_extension("xff");
        read_core(&path_with_xff_extension)
    }

    /// Writes a BTreeMap to a XFF file
    pub fn write<P>(path: P, data: BTreeMap<String, XffValue>) -> Result<(), NabuError> where P: AsRef<std::path::Path> {
        let path_with_xff_extension = path.as_ref().with_extension("xff");
        write_core(&path_with_xff_extension, data)
    }

    /// Creates a new BTreeMap
    pub fn new_core_store() -> BTreeMap<String, XffValue> {
        BTreeMap::new()
    }
}

#[cfg(feature = "key_value_store")]
/// Module to create a basic key-value database
pub mod key_value_store {
    use crate::{error::NabuError, features::key_value::store::NabuDB};

    /// Creates a new key-value database for in place operations
    ///
    /// Reads the content of a XFF file at the specified path and returns a `NabuDB` struct, which can be used for in place data manipulation and querying
    /// Please note that only XFF files written by the `save` function of `NabuDB` are supported
    ///
    /// # Arguments
    /// * `path` - The path to the file to write
    ///     
    /// # Example
    /// ```rust
    /// use nabu::key_value_store::new_nabudb;
    /// use nabu::xff::value::{XffValue, CommandCharacter, Data, Number};
    /// 
    /// let path = "xff-example-data/nabuDB_main_example.xff";
    /// let mut db = new_nabudb(path).unwrap();
    /// db.insert("key0".to_string(), XffValue::String("value0".to_string()));
    /// db.insert("key1".to_string(), XffValue::Number(Number::from(-42)));
    /// db.insert("key2".to_string(), XffValue::CommandCharacter(CommandCharacter::LineFeed));
    /// db.insert("key3".to_string(), XffValue::Data(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])));
    /// db.save();
    /// let read = new_nabudb(path).unwrap();
    /// assert_eq!(read.get("key0").unwrap(), db.get("key0").unwrap());
    /// assert_eq!(read.get("key1").unwrap(), db.get("key1").unwrap());
    /// assert_eq!(read.get("key2").unwrap(), db.get("key2").unwrap());
    /// assert_eq!(read.get("key3").unwrap(), db.get("key3").unwrap());
    /// ```
    pub fn new_nabudb<P>(path: P) -> Result<NabuDB, NabuError> where P: AsRef<std::path::Path> {
        NabuDB::new(path.as_ref().with_extension("xff"))
    }
}
