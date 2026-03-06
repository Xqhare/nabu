#![doc = include_str!("../README.md")]

mod error;

mod xff;

// Re-exported types - XffValue was moved out of Nabu and into Athena
pub use athena::{Array, CommandCharacter, Data, Number, Object, XffValue};

/// Most recent finalised version of XFF specification
const XFF_VERSION: u8 = 2;

// ----------------------------------------------------------
// Macros
// ----------------------------------------------------------

#[macro_export]
/// Macro to convert any value into a `XffValue`
///
/// ## Example
/// ```rust
/// use nabu::{XffValue, xff};
/// let value_int = xff!(42);
/// let value_string = xff!("hello mom");
/// let value_float = xff!(42.0);
/// let value_vec = xff!(vec![42]);
///
/// assert_eq!(value_int, XffValue::from(42));
/// assert_eq!(value_string, XffValue::from("hello mom"));
/// assert_eq!(value_float, XffValue::from(42.0));
/// assert_eq!(value_vec, XffValue::from(vec![42]));
/// ```
macro_rules! xff {
    ($value: expr) => {
        XffValue::from($value)
    };
}

#[macro_export]
/// Macro to convert any Vector into a `Vec<XffValue>`
///
/// ## Example
/// ```rust
/// use nabu::{XffValue, tvec_to_xff_value};
/// let vec = tvec_to_xff_value!(u8; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
/// let vec2 = tvec_to_xff_value!(i8; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
/// let ary: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
/// assert_eq!(vec, XffValue::from(ary.clone()));
/// assert_ne!(vec2, XffValue::from(ary));
/// ```
macro_rules! tvec_to_xff_value {
    ($t:ty; $($e:expr),*) => { XffValue::from(vec![$($e as $t),*] as Vec<$t>)};
}

/// Module to serialize and deserialize XFF files
///
/// # Example
/// ```rust
/// use nabu::serde::{read, write, remove_file};
/// use nabu::{CommandCharacter, Data, Number, XffValue};
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
    use crate::XFF_VERSION;
    use crate::XffValue;
    use crate::error::NabuError;
    use crate::xff::deserializer::deserialize_xff;
    use crate::xff::serializer::{serialize_xff, write_bytes_to_file};

    /// Reads the content of a XFF file and returns a XffValue
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
    /// use nabu::XffValue;
    ///
    /// let tmp = read("xff-example-data/v2.xff");
    /// assert!(tmp.is_ok());
    /// let data: XffValue = tmp.unwrap();
    /// println!("{}", data);
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
    /// Uses the most up to date version of the XFF specification.
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
    /// use nabu::{CommandCharacter, Data, Number, XffValue};
    ///
    /// let data = XffValue::String("hello mom".to_string());
    /// let tmp = write("xff-example-data/v2.xff", data.clone());
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
    /// Only use a Vector with more than one element if using version 0.
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
    /// use nabu::XffValue;
    ///
    /// let data = {
    ///     vec![
    ///         XffValue::String("hello mom".to_string()),
    ///     ]
    /// };
    /// let tmp = write_legacy("xff-example-data/v2.xff", data.clone(), 0);
    /// assert!(tmp.is_ok());
    /// ```
    pub fn write_legacy<P, D>(path: P, data: D, xff_version: u8) -> Result<(), NabuError>
    where
        P: AsRef<std::path::Path>,
        D: Into<Vec<XffValue>>,
    {
        let path_with_xff_extension = path.as_ref().with_extension("xff");
        let byte_data = serialize_xff(data.into(), xff_version)?;
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
    /// # use nabu::XffValue;
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
