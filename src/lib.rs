/*!
*/

mod error;

pub mod xff;

/// Most recent finalised version of XFF specification
const XFF_VERSION: u8 = 1;

/// Module to serialize and deserialize XFF files
///
/// # Example
/// ```rust
/// use nabu::serde::{read, write, remove_file};
/// use nabu::xff::value::{CommandCharacter, Data, Number, XffValue};
/// // No matter what the extension of the path you provide, it will be converted to .xff
/// let path = "xff-example-data/serde-main-example.txt";
/// let path_2 = "xff-example-data/serde-main-example.xff";
///
/// let data = XffValue::String("hello mom".to_string());
///
/// let write = write(path, data.clone());
/// assert!(write.is_ok());
/// let read = read(path_2);
/// assert!(read.is_ok());
/// let ok = read.unwrap();
/// assert_eq!(ok, data);
/// // delete file with the inbuilt remove_file function
/// remove_file(path_2).unwrap();
/// ```
pub mod serde {
    use crate::error::NabuError;
    use crate::xff::deserializer::deserialize_xff;
    use crate::xff::serializer::{serialize_xff, write_bytes_to_file};
    use crate::xff::value::XffValue;
    use crate::XFF_VERSION;

    /// Reads the content of a XFF file and returns a Vec of XffValues
    ///
    /// Because of the way v0 is implemented, it always returns a vector, for v1 it only has one element
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
    /// use nabu::xff::value::XffValue;
    ///
    /// let tmp = read("xff-example-data/v0.xff");
    /// assert!(tmp.is_ok());
    /// let data: XffValue = tmp.unwrap();
///     println!("{}", data);
    /// ```
    pub fn read<P>(path: P) -> Result<XffValue, NabuError>
    where
        P: AsRef<std::path::Path>,
    {
        let path_with_xff_extension = path.as_ref().with_extension("xff");
        deserialize_xff(&path_with_xff_extension)
    }

    /// Writes XffValues to a XFF file
    ///
    /// Supports the most up to date version of the XFF specification.
    /// To write v1, please supply only one element.
    ///
    /// To write legacy versions, please refer to `write_legacy`.
    ///
    /// # Arguments
    /// * `path` - The path to the file to write
    /// * `data` - The XffValue to write
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
    pub fn write<P, D>(path: P, data: D) -> Result<(), NabuError>
    where
        P: AsRef<std::path::Path>,
        D: Into<Vec<XffValue>>,
    {
        let path_with_xff_extension = path.as_ref().with_extension("xff");
        let byte_data = serialize_xff(data.into(), XFF_VERSION)?;
        write_bytes_to_file(&path_with_xff_extension, byte_data)
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
    pub fn write_legacy<P>(path: P, data: Vec<XffValue>, xff_version: u8) -> Result<(), NabuError>
    where
        P: AsRef<std::path::Path>,
    {
        let path_with_xff_extension = path.as_ref().with_extension("xff");
        let byte_data = serialize_xff(data, xff_version)?;
        write_bytes_to_file(&path_with_xff_extension, byte_data)
    }

    /// A convenience function to delete any XFF file from disk
    /// The file will be gone, take care!
    ///
    /// # Arguments
    /// * `path` - The path to the file to remove
    ///
    /// # Error
    /// Only errors if an IO error occurs
    ///
    /// # Example
    /// ```rust
    /// # use nabu::serde::write;
    /// # use nabu::xff::value::XffValue;
    ///
    /// # let data = {vec![XffValue::String("hello mom".to_string())]};
    /// # let _ = write("xff-example-data/remove.xff", data.clone());
    /// use nabu::serde::remove_file;
    ///
    /// let tmp = remove_file("xff-example-data/remove.xff");
    /// assert!(tmp.is_ok());
    /// ```
    pub fn remove_file<P>(path: P) -> Result<(), NabuError>
    where
        P: AsRef<std::path::Path>,
    {
        let path_with_xff_extension = path.as_ref().with_extension("xff");
        Ok(std::fs::remove_file(path_with_xff_extension)?)
    }
}

// -------------------------------------------------
//             FOR LEGACY USE ONLY
// -------------------------------------------------

// Remember to add any and all new features to this!
// It's literally a feature-gate for all features - leads to cleaner and leaner code I hope
#[cfg(any(
    doc,
    feature = "key_value_core",
    feature = "key_value_store",
    feature = "logging_wizard"
))]
pub mod features;

#[cfg(any(feature = "logging_wizard", doc))]
/// Module to create and manage a logging wizard
pub mod logging_wizard {
    pub use crate::features::logging_wizard::{Log, LogData, LoggingWizard};
}


#[cfg(any(doc, feature = "key_value_core"))]
/// LEGACY (v0) - Please consider using the inbuilt `OBJECT` type instead
///
/// Module to read and write a basic key-value store in the form of a `BTreeMap`
///
/// # Example
/// ```ignore
/// use std::{collections::BTreeMap, path::Path};
/// use nabu::features::key_value::core::read_core;
/// use nabu::xff::value::XffValue;
///
/// let data = read_core(&Path::new("xff-example-data/key_value_core.xff"));
/// assert!(data.is_ok());
/// let map: BTreeMap<String, XffValue> = data.unwrap();
/// ```
pub mod key_value_core {
    use std::collections::BTreeMap;

    use crate::{
        error::NabuError,
        features::key_value::core::{read_core, write_core},
        xff::value::XffValue,
    };

    /// LEGACY (v0) - Please consider using the inbuilt `OBJECT` type instead
    ///
    /// Reads the content of a XFF file and returns a BTreeMap
    /// Please note that only XFF files written by the `write` function of this module are supported
    ///
    /// # Arguments
    /// * `path` - The path to the file to read
    ///
    /// # Example
    /// ```ignore
    /// use nabu::features::key_value::core::read_core;
    /// use nabu::xff::value::XffValue;
    /// use std::collections::BTreeMap;
    /// use std::path::Path;
    ///
    /// let data = read_core(&Path::new("xff-example-data/key_value_core.xff"));
    /// assert!(data.is_ok());
    /// let map: BTreeMap<String, XffValue> = data.unwrap();
    /// ```
    pub fn read<P>(path: P) -> Result<BTreeMap<String, XffValue>, NabuError>
    where
        P: AsRef<std::path::Path>,
    {
        let path_with_xff_extension = path.as_ref().with_extension("xff");
        read_core(&path_with_xff_extension)
    }

    /// LEGACY (v0) - Please consider using the inbuilt `OBJECT` type instead
    ///
    /// Writes a BTreeMap to a XFF file
    ///
    /// # Arguments
    /// * `path` - The path to the file to write
    /// * `data` - The BTreeMap to write
    ///
    /// # Example
    /// ```ignore
    /// use nabu::key_value_core::{write, new_core_store};
    /// use nabu::xff::value::{XffValue, Number};
    ///
    /// let mut data = new_core_store();
    /// data.insert("key0".to_string(), XffValue::String("value0".to_string()));
    /// data.insert("key1".to_string(), XffValue::Number(Number::from(42)));
    ///
    /// let tmp = write("xff-example-data/key_value_core.xff", data.clone());
    /// assert!(tmp.is_ok());
    /// ```
    pub fn write<P>(path: P, data: BTreeMap<String, XffValue>) -> Result<(), NabuError>
    where
        P: AsRef<std::path::Path>,
    {
        let path_with_xff_extension = path.as_ref().with_extension("xff");
        write_core(&path_with_xff_extension, data)
    }

    /// LEGACY (v0) - Please consider using the inbuilt `OBJECT` type instead
    ///
    /// Creates a new BTreeMap
    ///
    /// # Example
    /// ```ignore
    /// use std::collections::BTreeMap;
    /// use nabu::{key_value_core::new_core_store, xff::value::XffValue};
    ///
    /// let data: BTreeMap<String, XffValue> = new_core_store();
    /// assert!(data.is_empty());
    /// ```
    pub fn new_core_store() -> BTreeMap<String, XffValue> {
        BTreeMap::new()
    }
}

#[cfg(any(feature = "key_value_store", doc))]
/// LEGACY (v0) - Please consider using the inbuilt `OBJECT` type instead
///
/// Module to create a basic key-value database
pub mod key_value_store {
    use crate::{error::NabuError, features::key_value::store::NabuDB};

    /// LEGACY (v0) - Please consider using the inbuilt `OBJECT` type instead
    ///
    /// Creates a new key-value database for in place operations
    ///
    /// Reads the content of a XFF file at the specified path and returns a `NabuDB` struct, which can be used for in place data manipulation and querying
    /// Please note that only XFF files written by the `save` function of `NabuDB` are supported
    ///
    /// # Arguments
    /// * `path` - The path to the file to write
    ///     
    /// # Example
    /// ```ignore
    /// use nabu::key_value_store::new_nabudb;
    /// use nabu::features::key_value::store::NabuDB;
    /// use nabu::xff::value::{XffValue, CommandCharacter, Data, Number};
    ///
    /// let path = "xff-example-data/nabuDB_main_example.xff";
    /// let mut db: NabuDB = new_nabudb(path).unwrap();
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
    pub fn new_nabudb<P>(path: P) -> Result<NabuDB, NabuError>
    where
        P: AsRef<std::path::Path>,
    {
        NabuDB::new(path.as_ref().with_extension("xff"))
    }
}
