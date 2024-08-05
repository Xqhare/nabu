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
/// use nabu::logging_wizard::LoggingWizard;
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
/// use nabu::logging_wizard::LoggingWizard;
/// use nabu::xff::value::XffValue;
///
/// let mut wizard = LoggingWizard::new("xff-example-data/logging_wizard.xff");
/// ```
///
/// You can split up the logs into multiple files by calling the `LoggingWiard::new` function with
/// a different path if you so desire, however the LoggingWiard is designed to work with a single
/// file, think of it more like a single .log file.
#[derive(Clone, Debug, PartialEq)]
pub struct LoggingWizard {
    /// This stores if the logs are to be appended or not
    append: bool,
    /// This stores the path of the file where the logs are written, this file would only be
    /// crated if not existent, otherwise a simple append would be used
    path: std::path::PathBuf,
    /// This stores all the logs in chronological order
    pub logs: Vec<Log>,
    /// This stores the length of the logs, provided for convenience
    ///
    /// e.g. if logs should be held in memory and written to disk as a batch, you could use this to call `save` if the len == 10
    pub logs_len: usize,
}

impl LoggingWizard {

    /// Removes the log at the specified index and returns it
    ///
    /// # Arguments
    /// * `index` - The index of the log to remove
    ///
    /// # Example
    /// ```rust
    /// use nabu::logging_wizard::{LoggingWizard, Log};
    /// use nabu::xff::value::XffValue;
    ///
    /// let mut wizard = LoggingWizard::new("xff-example-data/logging_wizard.xff");
    /// let log = Log::new();
    /// wizard.add_log(log);
    /// assert!(wizard.logs_len == 1);
    /// let get = wizard.get_log(0);
    /// assert!(wizard.logs_len == 0);
    /// assert!(get.is_some());
    /// ```
    pub fn get_log(&mut self, index: usize) -> Option<Log> {
        if index >= self.logs_len {
            return None
        }
        let out = self.logs.remove(index);
        self.logs_len -= 1;
        Some(out)
    }

    /// Creates a new LoggingWizard from disk
    ///
    /// # Arguments
    /// * `path` - The path to the file to read
    ///
    /// # Example
    /// ```rust
    /// use nabu::logging_wizard::LoggingWizard;
    ///
    /// let wizard = LoggingWizard::from_file("xff-example-data/logging_wizard.xff");
    /// assert!(wizard.is_ok());
    /// ```
    pub fn from_file<P>(path: P) -> Result<LoggingWizard, NabuError> where P: AsRef<std::path::Path> {
        let path = path.as_ref().to_path_buf().with_extension("xff");
        if path.exists() {
            read_log_wizard(path, false)
        } else {
            Ok(LoggingWizard { append: false, path, logs: Vec::new(), logs_len: 0 })
        }
    }

    /// Creates a new LoggingWizard without reading the file from disk to memory, appends all
    /// new logs to the end
    ///
    /// # Arguments
    /// * `path` - The path to the file to write
    ///
    /// # Example
    /// ```rust
    /// use nabu::logging_wizard::LoggingWizard;
    ///
    /// let wizard = LoggingWizard::new("xff-example-data/logging_wizard.xff");
    /// ```
    pub fn new<P>(path: P) -> LoggingWizard where P: AsRef<std::path::Path> {
        let path = path.as_ref().to_path_buf().with_extension("xff");
        LoggingWizard { append: true, path, logs: Vec::new(), logs_len: 0 }
    }

    /// Saves the LoggingWizard to disk
    ///
    /// # Example
    /// ```rust
    /// use nabu::logging_wizard::LoggingWizard;
    ///
    /// let mut wizard = LoggingWizard::new("xff-example-data/logging_wizard.xff");
    /// let write_result = wizard.save();
    /// assert!(write_result.is_ok());
    ///
    /// # // clean up
    /// # std::fs::remove_file("xff-example-data/logging_wizard.xff").unwrap();
    /// ```
    pub fn save(&mut self) -> Result<(), NabuError> {
        if self.append {
            match append_to_log_wizard(&self.path, &self.logs) {
                Ok(_) => {
                    self.logs.clear();
                    self.logs_len = 0;
                    Ok(())
                },
                Err(e) => {
                    Err(e)
                }
            }
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
    /// use nabu::logging_wizard::{LoggingWizard, Log};
    /// use nabu::xff::value::XffValue;
    ///
    /// let mut wizard = LoggingWizard::new("xff-example-data/logging_wizard.xff");
    /// let log = Log::new();
    /// wizard.add_log(log);
    /// assert!(wizard.logs_len == 1);
    /// ```
    pub fn add_log(&mut self, log: Log) {
        self.logs.push(log);
        self.logs_len = self.logs_len.saturating_add(1);
    }

    /// Adds a new log to the LoggingWizard and saves it to disk
    ///
    /// This is a convenience function that calls `add_log` and `save` in succession, and can
    /// be used to save every log as it is added, minimising memory usage by trading it with IO
    /// operations.
    ///
    /// # Arguments
    /// * `log` - The log to add
    ///
    /// # Example
    /// ```rust
    /// use nabu::logging_wizard::{LoggingWizard, Log};
    /// use nabu::xff::value::XffValue;
    ///
    /// let mut wizard = LoggingWizard::new("xff-example-data/logging_wizard.xff");
    /// let log = Log::new();
    /// let write_result = wizard.add_log_and_save(log);
    /// assert!(write_result.is_ok());
    /// assert!(wizard.logs_len == 0);
    ///
    /// # // clean up
    /// # std::fs::remove_file("xff-example-data/logging_wizard.xff").unwrap();
    /// ```
    pub fn add_log_and_save(&mut self, log: Log) -> Result<(), NabuError> {
        self.add_log(log);
        self.save()
    }

    /// Removes a log from the LoggingWizard
    ///
    /// # Arguments
    /// * `index` - The index of the log to remove
    ///
    /// # Example
    /// ```rust
    /// use nabu::logging_wizard::{LoggingWizard, Log};
    /// use nabu::xff::value::XffValue;
    ///
    /// let mut wizard = LoggingWizard::new("xff-example-data/logging_wizard.xff");
    /// let log = Log::new();
    /// wizard.add_log(log);
    /// wizard.remove_log(0);
    /// assert!(wizard.logs_len == 0);
    /// ```
    pub fn remove_log(&mut self, index: usize) -> Option<Log> {
        self.logs_len = self.logs_len.saturating_sub(1);
        if index >= self.logs_len {
            return None
        } else {
            return Some(self.logs.remove(index))
        }
    }
}

/// Stores all data of a single log. This can be as much data as you want.
#[derive(Clone, Debug, PartialEq)]
pub struct Log {
    /// Every data point of the log is stored here
    pub log_data: Vec<LogData>,
    /// The number of data points in the log
    pub log_data_len: usize,
}

impl From<Vec<LogData>> for Log {
    fn from(log_data: Vec<LogData>) -> Self {
        Log { log_data_len: log_data.len(), log_data }
    }
}

impl Default for Log {
    fn default() -> Self {
        Log { log_data: Default::default(), log_data_len: 0 }
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
    /// use nabu::logging_wizard::{Log, LogData};
    /// use nabu::xff::value::{XffValue, Number};
    ///
    /// let data = LogData::new("name", XffValue::Number(Number::from(42)), None);
    /// let mut log = Log::new();
    /// log.add_log_data(data);
    /// println!("{:?}", log);
    /// assert!(log.log_data_len == 1);
    /// ```
    pub fn add_log_data(&mut self, log_data: LogData) {
        self.log_data.push(log_data);
        self.log_data_len = self.log_data_len.saturating_add(1);
    }

    /// Removes a data point from the log
    ///
    /// # Arguments
    /// * `index` - The index of the data point to remove
    ///
    /// # Example
    /// ```rust
    /// use nabu::logging_wizard::{Log, LogData};
    /// use nabu::xff::value::{XffValue, Number};
    ///
    /// let data = LogData::new("name", XffValue::Number(Number::from(42)), None);
    /// let mut log = Log::new();
    /// log.add_log_data(data);
    /// log.remove_log_data(0);
    /// assert!(log.log_data_len == 0);
    /// ```
    pub fn remove_log_data(&mut self, index: usize) {
        self.log_data.remove(index);
        self.log_data_len = self.log_data_len.saturating_sub(1);
    }

    /// Creates a new Log
    /// Alternatively use the `default` function
    ///
    /// Used to populate a LoggingWizard
    ///
    /// # Example
    /// ```rust
    /// use nabu::logging_wizard::Log;
    /// use nabu::xff::value::XffValue;
    ///
    /// let log = Log::new();
    /// assert!(log.log_data_len == 0);
    /// ```
    pub fn new() -> Log {
        Log { log_data: Vec::new() , log_data_len: 0 }
    }
}

/// Stores a single data point of the log
#[derive(Clone, Debug, PartialEq)]
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
    /// # Example
    /// ```rust
    /// use nabu::logging_wizard::LogData;
    /// use nabu::xff::value::{XffValue, Number};
    ///
    /// let data = LogData::create("name", XffValue::Number(Number::from(42)), None);
    /// assert!(data.name == "name");
    /// assert!(data.value == XffValue::Number(Number::from(42)));
    /// assert!(data.optional_metadata.len() == 0);
    /// ```
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
    /// # Example
    /// ```rust
    /// use nabu::logging_wizard::LogData;
    /// use nabu::xff::value::{XffValue, Number};
    ///
    /// let data = LogData::new("name", XffValue::Number(Number::from(42)), None);
    /// assert!(data.name == "name");
    /// assert!(data.value == XffValue::Number(Number::from(42)));
    /// assert!(data.optional_metadata.is_empty());
    /// ```
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
    ///
    /// # Example
    /// ```rust
    /// use std::collections::BTreeMap;
    /// use nabu::logging_wizard::LogData;
    /// use nabu::xff::value::{XffValue, Number};
    ///
    /// let mut data = LogData::new("name", XffValue::Number(Number::from(42)), Some(BTreeMap::new()));
    /// data.add_metadata("time", "12:34");
    /// data.add_metadata("extension", "jpg");
    /// assert!(data.optional_metadata.contains_key("time"));
    /// assert!(data.optional_metadata.contains_key("extension"));
    /// ```
    pub fn add_metadata<S: Into<String>>(&mut self, key: S, value: S) {
        self.optional_metadata.insert(key.into(), value.into());
    }

    /// Removes a metadata entry
    ///
    /// # Arguments
    /// * `key` - The key of the metadata entry
    ///
    /// # Example
    /// ```rust
    /// use std::collections::BTreeMap;
    /// use nabu::logging_wizard::LogData;
    /// use nabu::xff::value::{XffValue, Number};
    ///
    /// let mut data = LogData::new("name", XffValue::Number(Number::from(42)), Some(BTreeMap::new()));
    /// data.add_metadata("extension", "jpg");
    /// data.remove_metadata("extension");
    /// assert!(!data.optional_metadata.contains_key("extension"));
    /// ```
    pub fn remove_metadata(&mut self, key: &str) {
        self.optional_metadata.remove(key);
    }
}


