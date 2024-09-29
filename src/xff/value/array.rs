use std::ops::Index;

use super::XffValue;

#[derive(Debug, Clone, PartialEq)]
/// An array of XFF values.
///
/// Can be crated with `Array::from()` or `Array::new()`.
///
/// Most functionality needed for interacting with the underlying vector is provided trough the struct itself.
///
/// Access to the underlying map is provided through the `into_vec()` method.
///
/// `Array` implements `From<Vec<T>>`, where `T` can be converted into a `XffValue`.
/// This means that any value that can be put into `XffValue::from()` can be put into a vector and
/// passed into `Array::from()`.
///
/// # Examples
/// ```rust
/// use nabu::xff::value::{XffValue, Array};
///
/// let mut array1 = Array::from(vec![
///     XffValue::from("hi mom!"),
///     XffValue::from(42.69),
///     XffValue::from(42.69),
/// ]);
///
/// assert_eq!(array1.len(), 3);
/// array1.clear();
/// assert!(array1.is_empty());
///
/// let vector = array1.into_vec();
/// assert!(vector.is_empty());
///
/// let mut array2 = Array::new();
/// array2.push(XffValue::from("hello mom!"));
/// array2.push(XffValue::from(420.69));
/// array2.push(XffValue::from(420.69));
///
/// assert_eq!(array2.len(), 3);
/// let value_0 = array2.remove(0);
/// assert_eq!(value_0, XffValue::from("hello mom!"));
/// assert_eq!(array2.len(), 2);
///
/// let value_1 = array2.get(1).unwrap();
/// assert_eq!(value_1, &XffValue::from(420.69));
///
/// let value_2 = array2.pop().unwrap();
/// assert_eq!(value_2, XffValue::from(420.69));
/// assert_eq!(array2.len(), 1);
///
/// array2.insert(0, XffValue::from("hello dad!"));
/// assert_eq!(array2.len(), 2);
///
/// assert!(array2.contains(&XffValue::from(420.69)));
///
/// for value in array2.iter() {
///     println!("{}", value);
/// }
/// ```
pub struct Array {
    /// An array of XFF values of arbitrary length
    pub values: Vec<XffValue>,
}

// -----------------------------------------------------------
//                     General implementations
// -----------------------------------------------------------

impl Array {
    /// Creates a new and empty `Array`
    pub fn new() -> Self {
        Array { values: Vec::new() }
    }

    /// Returns the `Array` as a vector
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::{XffValue, Array};
    ///
    /// let mut array = Array::from(vec![
    ///     XffValue::from("hi mom!"),
    ///     XffValue::from(42.69),
    ///     XffValue::from(42.69),
    /// ]);
    ///
    /// let vector = array.into_vec();
    /// assert_eq!(vector, vec![
    ///     XffValue::from("hi mom!"),
    ///     XffValue::from(42.69),
    ///     XffValue::from(42.69),
    /// ]);
    /// ```
    pub fn into_vec(&self) -> Vec<XffValue> {
        self.values.clone()
    }

    /// Returns the length of the `Array`
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::{XffValue, Array};
    ///
    /// let mut array = Array::from(vec![
    ///     XffValue::from("hi mom!"),
    ///     XffValue::from(42.69),
    ///     XffValue::from(42.69),
    /// ]);
    ///
    /// assert_eq!(array.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns `true` if the `Array` is empty
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::{XffValue, Array};
    ///
    /// let mut array = Array::new();
    ///
    /// assert!(array.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Clears the `Array`
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::{XffValue, Array};
    ///
    /// let mut array = Array::from(vec![
    ///     XffValue::from("hi mom!"),
    ///     XffValue::from(42.69),
    ///     XffValue::from(42.69),
    /// ]);
    ///
    /// assert_eq!(array.len(), 3);
    ///
    /// array.clear();
    ///
    /// assert_eq!(array.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.values.clear();
    }

    /// Pushes a value onto the `Array`
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::{XffValue, Array};
    ///
    /// let mut array = Array::new();
    ///
    /// array.push(XffValue::from("hi mom!"));
    /// array.push(XffValue::from(42.69));
    /// array.push(XffValue::from(42.69));
    ///
    /// assert_eq!(array.len(), 3);
    /// ```
    pub fn push(&mut self, value: XffValue) {
        self.values.push(value);
    }

    /// Pops a value off the end of the `Array`
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::{XffValue, Array};
    ///
    /// let mut array = Array::from(vec![
    ///     XffValue::from("hi mom!"),
    ///     XffValue::from(42.69),
    ///     XffValue::from(42.69),
    /// ]);
    ///
    /// assert_eq!(array.len(), 3);
    ///
    /// let value = array.pop();
    /// assert_eq!(value, Some(XffValue::from(42.69)));
    /// assert_eq!(array.len(), 2);
    /// ```
    pub fn pop(&mut self) -> Option<XffValue> {
        self.values.pop()
    }

    /// Inserts a value at a given index
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::{XffValue, Array};
    ///
    /// let mut array = Array::new();
    ///
    /// array.insert(0, XffValue::from("hi mom!"));
    /// array.insert(1, XffValue::from(42.69));
    /// array.insert(2, XffValue::from(42.69));
    ///
    /// assert_eq!(array.len(), 3);
    /// ```
    pub fn insert(&mut self, index: usize, value: XffValue) {
        self.values.insert(index, value);
    }

    /// Removes a value at a given index
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::{XffValue, Array};
    ///
    /// let mut array = Array::from(vec![
    ///     XffValue::from("hi mom!"),
    ///     XffValue::from(42.69),
    ///     XffValue::from(42.69),
    /// ]);
    ///
    /// assert_eq!(array.len(), 3);
    ///
    /// let value = array.remove(1);
    /// assert_eq!(value, XffValue::from(42.69));
    /// assert_eq!(array.len(), 2);
    /// ```
    pub fn remove(&mut self, index: usize) -> XffValue {
        self.values.remove(index)
    }

    /// Gets a reference to a value at a given index
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::{XffValue, Array};
    ///
    /// let mut array = Array::from(vec![
    ///     XffValue::from("hi mom!"),
    ///     XffValue::from(42.69),
    ///     XffValue::from(42.69),
    /// ]);
    ///
    /// assert_eq!(array.len(), 3);
    ///
    /// let value = array.get(1);
    /// assert_eq!(value, Some(&XffValue::from(42.69)));
    /// ```
    pub fn get(&self, index: usize) -> Option<&XffValue> {
        self.values.get(index)
    }

    /// Returns `true` if the `Array` contains the supplied value
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::{XffValue, Array};
    ///
    /// let mut array = Array::from(vec![
    ///     XffValue::from("hi mom!"),
    ///     XffValue::from(42.69),
    ///     XffValue::from(42.69),
    /// ]);
    ///
    /// assert_eq!(array.len(), 3);
    ///
    /// let value = array.contains(&XffValue::from(42.69));
    /// assert_eq!(value, true);
    /// ```
    pub fn contains(&self, value: &XffValue) -> bool {
        self.values.contains(value)
    }

    /// Returns an iterator over the values in the array
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::{XffValue, Array};
    ///
    /// let array = Array::from(vec![
    ///     XffValue::from("hi mom!"),
    ///     XffValue::from(42.69),
    ///     XffValue::from(42.69),
    /// ]);
    ///
    /// let mut iter = array.iter();
    /// assert_eq!(iter.next(), Some(&XffValue::from("hi mom!")));
    /// assert_eq!(iter.next(), Some(&XffValue::from(42.69)));
    /// assert_eq!(iter.next(), Some(&XffValue::from(42.69)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> std::slice::Iter<'_, XffValue> {
        self.values.iter()
    }
}

// -----------------------------------------------------------
//                     From implementations
// -----------------------------------------------------------

impl<V> From<Vec<V>> for Array
where
    V: Into<XffValue>,
{
    fn from(values: Vec<V>) -> Self {
        Array {
            values: values.into_iter().map(|v| v.into()).collect(),
        }
    }
}

// -----------------------------------------------------------
//                     Iterator implementations
// -----------------------------------------------------------

impl<T> FromIterator<T> for Array
where
    T: Into<XffValue>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Array {
            values: iter.into_iter().map(|v| v.into()).collect(),
        }
    }
}

impl IntoIterator for Array {
    type Item = XffValue;
    type IntoIter = std::vec::IntoIter<XffValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

// -----------------------------------------------------------
//                     Index implementations
// -----------------------------------------------------------

impl Index<usize> for Array {
    type Output = XffValue;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

// -----------------------------------------------------------
//                     Display implementation
// -----------------------------------------------------------

impl std::fmt::Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, value) in self.values.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", value)?;
        }
        write!(f, "]")
    }
}
