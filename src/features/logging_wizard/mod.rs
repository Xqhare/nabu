mod serde;

use crate::{xff::value::XffValue, features::logging_wizard::serde::{read_log_wizard, append_to_log_wizard, write_log_wizard}, error::NabuError};

use std::collections::BTreeMap;

/// Module to create, read and write XFF-Logging files
///
/// # Creating a LoggingWizard
/// There are several ways to create a LoggingWizard, depending on your need.
///
/// Should you want to load all previous logs, you can use the `LoggingWizard::from_file` function.
///
/// This function will decode the file and return a LoggingWizard.
/// Should no file exist, a new LoggingWizard will be created.
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
/// The file is not loaded at all until you call the `LoggingWizard::save` function.
/// During saving the file would not be decoded from the byte form, but loaded into memory and all new logs would be appended
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
#[derive(Clone, Debug)]
pub struct LoggingWizard {
    /// This stores if the logs are to be appended or not
    append: bool,
    /// This stores the path of the file where the logs are written, this file would only be
    /// crated if not existent, otherwise a simple append would be used
    path: std::path::PathBuf,
    /// This stores all the logs in chronological order
    pub logs: Vec<Log>,
}

impl LoggingWizard {
    /// Creates a new LoggingWizard from disk
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
        let path = path.as_ref().to_path_buf().with_extension("xff");
        if path.exists() {
            read_log_wizard(path, false)
        } else {
            Ok(LoggingWizard { append: false, path, logs: Vec::new() })
        }
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
    pub fn new<P>(path: P) -> LoggingWizard where P: AsRef<std::path::Path> {
        let path = path.as_ref().to_path_buf().with_extension("xff");
        LoggingWizard { append: true, path, logs: Vec::new() }
    }

    /// Saves the LoggingWizard to disk
    ///
    /// # Example
    /// ```rust
    /// use std::path::Path;
    /// use nabu::features::logging_wizard::LoggingWizard;
    ///
    /// let path = Path::new("xff-example-data/logging_wizard.xff");
    /// let mut wizard = LoggingWizard::new(path).unwrap();
    /// wizard.save();
    /// ```
    pub fn save(&self) -> Result<(), NabuError> {
        if self.append {
            append_to_log_wizard(&self.path, &self.logs)
        } else {
            write_log_wizard(&self.path, &self.logs)
        }
    }

    /// Adds a new log to the LoggingWizard
    ///
    /// # Arguments
    /// * `log` - The log to add
    ///
    /// # Example
    /// ```rust
    /// use std::path::Path;
    /// use nabu::features::logging_wizard::LoggingWizard;
    /// use nabu::xff::value::XffValue;
    ///
    /// let path = Path::new("xff-example-data/logging_wizard.xff");
    /// let mut wizard = LoggingWizard::new(path).unwrap();
    /// let log = Log::new();
    /// wizard.add_log(log);
    /// ```
    pub fn add_log(&mut self, log: Log) {
        self.logs.push(log);
    }
}

/// Stores all data of a single log. This can be as much data as you want.
#[derive(Clone, Debug)]
pub struct Log {
    /// Every data point of the log is stored here
    pub log_data: Vec<LogData>,
}

impl Default for Log {
    fn default() -> Self {
        Log { log_data: Default::default() }
    }
}

impl Log {

    /// Adds a new data point to the log
    ///
    /// To create a new data point, use the `LogData` struct
    /// 
    /// # Arguments
    /// * `log_data` - The data point to add
    ///
    /// # Example
    /// ```rust
    /// use nabu::features::logging_wizard::LogData;
    /// use nabu::xff::value::XffValue;
    ///
    /// let data = LogData::new("name", XffValue::Number(42), None);
    /// let mut log = Log::new();
    /// log.add_log_data(data);
    /// ```
    pub fn add_log_data(&mut self, log_data: LogData) {
        self.log_data.push(log_data);
    }

    /// Creates a new Log
    /// Alternatively use the `default` function
    ///
    /// Used to populate a LoggingWizard
    pub fn new() -> Log {
        Log { log_data: Vec::new() }
    }
}

/// Stores a single data point of the log
#[derive(Clone, Debug)]
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
    /// Instead of using a Optional type, an empty BTreeMap is used, this way adding new
    /// metadata entries is very easy
    pub optional_metadata: BTreeMap<String, String>,
}

impl LogData {

    /// Creates a new LogData from name, value and optional metadata
    /// Alternatively use the `new` function
    ///
    /// Used to populate a Log
    ///
    /// # Arguments
    /// * `name` - The name of the data point
    /// * `value` - The value of the data point
    /// * `optional_metadata` - The optional metadata of the data point
    ///
    pub fn create<T: Into<String>>(name: T, value: XffValue, optional_metadata: Option<BTreeMap<T, T>>) -> LogData {
        match optional_metadata {
            Some(metadata) => LogData { name: name.into(), value, optional_metadata: metadata.into_iter().map(|(k, v)| (k.into(), v.into())).collect() },
            None => LogData { name: name.into(), value, optional_metadata: BTreeMap::new() },
        }
    }

    /// Creates a new LogData from name, value and optional metadata
    /// Alternatively use the `create` function
    ///
    /// Used to populate a Log
    ///
    /// # Arguments
    /// * `name` - The name of the data point
    /// * `value` - The value of the data point
    /// * `optional_metadata` - The optional metadata of the data point
    ///
    pub fn new<T: Into<String>>(name: T, value: XffValue, optional_metadata: Option<BTreeMap<T, T>>) -> LogData {
        Self::create(name, value, optional_metadata)
    }

    /// Adds a new metadata entry
    ///
    /// # Arguments
    /// * `key` - The key of the metadata entry
    /// * `value` - The value of the metadata entry
    ///
    /// Both key and value have to be ASCII strings
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.optional_metadata.insert(key, value);
    }
}


