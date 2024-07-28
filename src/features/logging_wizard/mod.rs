mod serde;

use crate::{xff::value::{XffValue, CommandCharacter}, features::logging_wizard::serde::read_log_wizard, error::NabuError};

use std::collections::BTreeMap;

/// Module to create, read and write XFF-Logging files
///
/// # Creating a LoggingWizard
/// There are several ways to create a LoggingWizard, depending on your need.
///
/// Should you want to load all previous logs, you can use the `LoggingWizard::from_file` function.
///
/// ## Example
/// ```rust
/// use nabu::features::logging_wizard::LoggingWizard;
/// use nabu::xff::value::XffValue;
///
/// let mut wizard = LoggingWizard::from_file("xff-example-data/logging_wizard.xff");
/// assert!(wizard.is_ok());
/// ```
///
/// If you want to create a new LoggingWizard or want to add logs to an existing one, you can use the `LoggingWizard::new` function.
/// Here the file would not be decoded, but loaded into memory and all new logs would be appended
/// to the end.
/// If the supplied file does not exist, it would be created.
///
/// ## Example
/// ```rust
/// use nabu::features::logging_wizard::LoggingWizard;
/// use nabu::xff::value::XffValue;
///
/// let mut wizard = LoggingWizard::new("xff-example-data/logging_wizard.xff");
/// assert!(wizard.is_ok());
/// ```
///
/// You can split up the logs into multiple files by calling the `LoggingWiard::new` function with
/// a different path if you so desire, however the LoggingWiard is designed to work with a single
/// file, think of it more like a single .log file.
pub struct LoggingWizard {
    /// This stores the path of the file where the logs are written, this file would only be
    /// crated if not existent, otherwise a simple append would be used
    path: std::path::PathBuf,
    /// This stores all the logs in chronological order
    pub logs: Vec<Log>,
}

impl LoggingWizard {
    /// Creates a new LoggingWizard from a file
    ///
    /// # Arguments
    /// * `path` - The path to the file to read
    ///
    /// # Example
    /// ```rust
    /// use std::path::Path;
    /// use nabu::features::logging_wizard::LoggingWizard;
    ///
    /// let path = Path::new("xff-example-data/logging_wizard.xff");
    /// let wizard = LoggingWizard::from_file(path);
    /// assert!(wizard.is_ok());
    /// ```
    pub fn from_file<P>(path: P) -> Result<LoggingWizard, NabuError> where P: AsRef<std::path::Path> {
        let data = read_log_wizard(path)?;
        todo!()
    }

    /// Creates a new LoggingWizard
    ///
    /// # Arguments
    /// * `path` - The path to the file to write
    ///
    /// # Example
    /// ```rust
    /// use std::path::Path;
    /// use nabu::features::logging_wizard::LoggingWizard;
    ///
    /// let path = Path::new("xff-example-data/logging_wizard.xff");
    /// let wizard = LoggingWizard::new(path);
    /// assert!(wizard.is_ok());
    /// ```
    pub fn new<P>(path: P) -> Result<LoggingWizard, NabuError> where P: AsRef<std::path::Path> {
        let path = path.as_ref().to_path_buf();
        todo!()
    }
}

/// Stores all data of a single log. This can be as much data as you want.
pub struct Log {
    /// Every data point of the log is stored here
    pub log_data: Vec<LogData>,
}

/// Stores a single data point of the log
pub struct LogData {
    /// The name of the data point
    pub name: String,
    /// The value of the data point
    pub value: XffValue,
    /// The optional metadata of the data point
    /// E.g. some kind of picture is saved inside the value, then you could store the extension
    /// here in the form of `("extension", "jpg")`
    ///
    /// There is no limit on the number of metadata entries, but they have to be ASCII strings
    pub optional_metadata: BTreeMap<String, String>,
}


