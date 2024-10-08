#[derive(Debug, Clone, PartialEq)]
/// A data value, used to store arbitrary data.
///
/// Can be created from a `Vec<u8>` using the `From` trait.
///
/// # Example
/// ```rust
/// use nabu::Data;
///
/// let mut data = Data::from(vec![1, 2, 3, 4, 5]);
///
/// assert_eq!(data.len(), 5);
///
/// let vector = data.clone().into_vec();
/// assert_eq!(vector, vec![1, 2, 3, 4, 5]);
///
/// data.clear();
/// assert!(data.is_empty());
/// ```
pub struct Data {
    /// The actual data
    pub data: Vec<u8>,
    /// The length of the data
    pub len: usize,
}

// -----------------------------------------------------------
//                     General implementations
// -----------------------------------------------------------

impl Data {
    /// Clears the data
    ///
    /// # Example
    /// ```rust
    /// use nabu::Data;
    ///
    /// let mut data = Data::from(vec![1, 2, 3, 4, 5]);
    /// data.clear();
    /// assert!(data.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.data.clear();
        self.len = 0
    }

    /// Returns the length of the data
    ///
    /// # Example
    /// ```rust
    /// use nabu::Data;
    ///
    /// let data = Data::from(vec![1, 2, 3, 4, 5]);
    ///
    /// assert_eq!(data.len(), 5);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the data is empty
    ///
    /// # Example
    /// ```rust
    /// use nabu::Data;
    ///
    /// let data = Data::from(vec![]);
    /// assert!(data.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the data as a vector of bytes
    ///
    /// # Example
    /// ```rust
    /// use nabu::Data;
    ///
    /// let data = Data::from(vec![1, 2, 3, 4, 5]);
    ///
    /// let vector = data.into_vec();
    /// assert_eq!(vector, vec![1, 2, 3, 4, 5]);
    /// ```
    pub fn into_vec(self) -> Vec<u8> {
        self.data
    }
}

// -----------------------------------------------------------
//                     From implementations
// -----------------------------------------------------------

impl<D> From<D> for Data where D: Into<Vec<u8>> {
    fn from(data: D) -> Self {
        let data = data.into();
        Data {
            len: data.len(),
            data,
        }
    }
}

// -----------------------------------------------------------
//                     Display implementation
// -----------------------------------------------------------

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // it is a vector of bytes... debug is enough
        write!(f, "{:?}", &self.data)
    }
}
