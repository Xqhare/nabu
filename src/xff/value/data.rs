#[derive(Debug, Clone, PartialEq)]
/// A data value
/// contains `data` and `len`
///
/// Can be converted from `Vec<u8>` using the `From` trait
pub struct Data {
    /// The actual data
    pub data: Vec<u8>,
    /// The length of the data
    pub len: usize,
}

impl From<Vec<u8>> for Data {
    fn from(data: Vec<u8>) -> Self {
        Data {
            data: data.clone(),
            len: data.len(),
        }
    }
}

