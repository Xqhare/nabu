use crate::{serde::{read, write}, xff::value::XffValue, error::NabuError};

pub fn read_log_file<P>(path: P) -> Result<Vec<String>, NabuError> where P: AsRef<std::path::Path> {
    
}
