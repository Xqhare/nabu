use std::collections::{BTreeMap, HashMap};

pub use cmd_char::CommandCharacter;
pub use data::Data;
pub use num::Number;

pub mod cmd_char;
pub mod data;
pub mod num;

#[derive(Debug, Clone, PartialEq)]
/// An enum for the different types of XFF values.
///
/// Many From traits are implemented for convenience on `XffValue` directly.
/// 
/// Directly stored data, like `String`, `Array`, `Object`, `Booleans` and `Null` have convenience
/// functions implemented on `XffValue`.
///
/// `Data` and `Number` have convenience functions implemented on their respective types
///
/// All variants of `XffValue` are `Clone`able and have `is_` functions implemented.
/// E.g. `is_string()`, `is_number()`, etc.
///
/// All variants have also `into_` functions implemented to retrieve the wrapped data inside.
/// E.g. `into_string()`, `into_array()`, etc.
///
/// For more information please refer to the readme, or the documentation of the functiuon or type.
///
/// Deprecated and kept for compatibility with v0:
///
/// `CommandCharacter` is an enum representing a single ASCII command or control character
/// `ArrayCmdChar` is a list of `CommandCharacter`s and seldom used in writing XFF files, but never in reading them
pub enum XffValue {
    /// A string value
    String(String),
    /// A numeric value
    Number(Number),
    /// An array of XFF values of arbitrary length
    Array(Vec<XffValue>),
    /// An object of string keys and XffValue values
    Object(BTreeMap<String, XffValue>),
    /// A data value, holding arbitrary bytes
    Data(Data),
    /// A boolean value, true or false
    Boolean(bool),
    /// A null value, a.k.a. `None`, `Nill` or `nothing`
    Null,
    /// Deprecated
    /// Only used in v0, needed for legacy usage
    /// A command character is represented by the `CommandCharacter` enum
    CommandCharacter(CommandCharacter),
    /// Deprecated
    /// Only used in v0, needed for legacy usage
    /// An array of `CommandCharacter`s
    ArrayCmdChar(Vec<CommandCharacter>),
}

// -----------------------------------------------------------
//                     General implementations
// -----------------------------------------------------------

impl XffValue {
    /// Returns the value as a string
    ///
    /// Only works on `XffValue::String` and `XffValue::Number`.
    /// Returns `None` for all other variants
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::{XffValue, Number, CommandCharacter};
    ///
    /// let string_value = XffValue::from("hello mom!");
    /// let num_value = XffValue::from(42.69);
    /// let cmd_char_value = XffValue::from(CommandCharacter::Null);
    ///
    /// assert_eq!(string_value.into_string(), Some("hello mom!".to_string()));
    /// assert_eq!(num_value.into_string(), Some("42.69".to_string()));
    /// assert_eq!(cmd_char_value.into_string(), None);
    /// ```
    pub fn into_string(&self) -> Option<String> {
        match self {
            XffValue::String(s) => Some(s.clone()),
            XffValue::Number(n) => Some(n.as_string()),
            _ => None,
        }
    }

    /// Returns the value as a number if it is a `XffValue::Number`
    /// Returns `None` for all other variants
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::{XffValue, Number};
    ///
    /// let num_value = XffValue::from(42.69);
    /// let string_value = XffValue::from("hello mom!");
    ///
    /// assert_eq!(string_value.into_number(), None);
    /// assert_eq!(num_value.into_number(), Some(Number::from(42.69)));
    /// ```
    pub fn into_number(&self) -> Option<Number> {
        match self {
            XffValue::Number(n) => Some(n.clone()),
            _ => None,
        }
    }

    /// Returns the value as an array if it is a `XffValue::Array`
    /// Returns `None` for all other variants
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::XffValue;
    ///
    /// let vec_value = XffValue::from([XffValue::from("hello mom!"), XffValue::from(42.69)]);
    /// let num_value = XffValue::from(42.69);
    ///
    /// assert_eq!(num_value.into_array(), None);
    /// assert_eq!(vec_value.into_array(), Some(XffValue::from([XffValue::from("hello mom!"), XffValue::from(42.69)]));
    /// ```
    pub fn into_array(&self) -> Option<Vec<XffValue>> {
        match self {
            XffValue::Array(a) => Some(a.clone()),
            _ => None,
        }
    }

    /// Returns the value as an object if it is a `XffValue::Object`
    /// Returns `None` for all other variants
    ///
    /// # Example
    /// ```rust
    /// use std::collections::BTreeMap;
    /// use nabu::xff::value::{XffValue, Number};
    ///
    /// let map = BTreeMap::from([
    ///     ("key0".to_string(), XffValue::from("value0")),
    ///     ("key1".to_string(), XffValue::from(42.69)),
    /// ]);
    ///
    /// let map_value = XffValue::from(map.clone());
    /// let num_value = XffValue::from(42.69);
    ///
    /// assert_eq!(num_value.into_object(), None);
    /// assert_eq!(map_value.into_object(), Some(XffValue::from(map)));
    ///
    /// ```
    pub fn into_object(&self) -> Option<BTreeMap<String, XffValue>> {
        match self {
            XffValue::Object(o) => Some(o.clone()),
            _ => None,
        }
    }

    /// Returns the value as a data value if it is a `XffValue::Data`
    /// Returns `None` for all other variants
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::{XffValue, Data};
    ///
    /// let data_value = XffValue::from(vec![1, 2, 3]);
    /// let num_value = XffValue::from(42.69);
    ///
    /// assert_eq!(num_value.into_data(), None);
    /// assert_eq!(data_value.into_data(), Some(XffValue::from(vec![1, 2, 3])));
    /// ```
    pub fn into_data(&self) -> Option<Data> {
        match self {
            XffValue::Data(d) => Some(d.clone()),
            _ => None,
        }
    }

    /// Returns the value as a boolean if it is a `XffValue::Boolean`
    /// Returns `None` for all other variants
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::XffValue;
    ///
    /// let bool_value_true = XffValue::from(true);
    /// let bool_value_false = XffValue::from(false);
    /// let num_value = XffValue::from(42.69);
    ///
    /// assert_eq!(num_value.into_boolean(), None);
    /// assert_eq!(bool_value.into_boolean(), Some(true));
    /// assert_eq!(bool_value_false.into_boolean(), Some(false));
    /// ```
    pub fn into_boolean(&self) -> Option<bool> {
        match self {
            XffValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns null if it is a `XffValue::Null`
    /// Returns `None` for all other variants
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::XffValue;
    ///
    /// let null_value = XffValue::Null;
    /// let num_value = XffValue::from(42.69);
    ///
    /// assert_eq!(num_value.into_null(), Some(()));
    /// assert_eq!(null_value.into_null(), None);
    /// ```
    pub fn into_null(&self) -> Option<()> {
        match self {
            XffValue::Null => None,
            _ => Some(()),
        }
    }

    /// Checks if the value is a string, returns `true` if it is.
    /// Returns `false` for all other variants.
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::XffValue;
    ///
    /// let string_value = XffValue::from("hello mom!");
    /// let num_value = XffValue::from(42.69);
    ///
    /// assert!(!num_value.is_string());
    /// assert!(string_value.is_string());
    /// ```
    pub fn is_string(&self) -> bool {
        matches!(self, XffValue::String(_))
    }

    /// Checks if the value is a number, returns `true` if it is.
    /// Returns `false` for all other variants.
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::XffValue;
    ///
    /// let number_value = XffValue::from(42.69);
    /// let string_value = XffValue::from("hello mom!");
    ///
    /// assert!(!string_value.is_number());
    /// assert!(number_value.is_number());
    /// ```
    pub fn is_number(&self) -> bool {
        matches!(self, XffValue::Number(_))
    }

    /// Checks if the value is an array, returns `true` if it is.
    /// Returns `false` for all other variants.
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::XffValue;
    ///
    /// let array_value = XffValue::from(vec![XffValue::from("hello mom!"), XffValue::from(42.69)]);
    /// let string_value = XffValue::from("hello mom!");
    ///
    /// assert!(!string_value.is_array());
    /// assert!(array_value.is_array());
    /// ```
    pub fn is_array(&self) -> bool {
        matches!(self, XffValue::Array(_))
    }

    /// Checks if the value is an object, returns `true` if it is.
    /// Returns `false` for all other variants.
    ///
    /// # Example
    /// ```rust
    /// use std::collections::BTreeMap;
    /// use nabu::xff::value::XffValue;
    ///
    /// let map = BTreeMap::from([XffValue::from("hello mom!"), XffValue::from(vec![1, 2, 3])]);
    ///
    /// let object_value = XffValue::from(map);
    /// let string_value = XffValue::from("hello mom!");
    ///
    /// assert!(!string_value.is_object());
    /// assert!(object_value.is_object());
    /// ```
    pub fn is_object(&self) -> bool {
        matches!(self, XffValue::Object(_))
    }

    /// Checks if the value is data, returns `true` if it is.
    /// Returns `false` for all other variants.
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::XffValue;
    ///
    /// let data_value = XffValue::from(vec![1, 2, 3]);
    /// let string_value = XffValue::from("hello mom!");
    ///
    /// assert!(!string_value.is_data());
    /// assert!(data_value.is_data());
    /// ```
    pub fn is_data(&self) -> bool {
        matches!(self, XffValue::Data(_))
    }

    /// Checks if the value is a boolean, returns `true` if it is.
    /// Returns `false` for all other variants.
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::XffValue;
    ///
    /// let boolean_value = XffValue::from(true);
    /// let string_value = XffValue::from("hello mom!");
    ///
    /// assert!(!string_value.is_boolean());
    /// assert!(boolean_value.is_boolean());
    /// ```
    pub fn is_boolean(&self) -> bool {
        matches!(self, XffValue::Boolean(_))
    }

    /// Checks if the value is both a boolean and true, returns `true` if it is.
    /// Returns `false` for all other variants.
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::XffValue;
    ///
    /// let boolean_value_true = XffValue::from(true);
    /// let boolean_value_false = XffValue::from(false);
    /// let string_value = XffValue::from("hello mom!");
    ///
    /// assert!(!string_value.is_true());
    /// assert!(!boolean_value_false.is_true());
    /// assert!(boolean_value_true.is_true());
    /// ```
    pub fn is_true(&self) -> bool {
        matches!(self, XffValue::Boolean(true))
    }

    /// Checks if the value is both a boolean and false, returns `true` if it is.
    /// Returns `false` for all other variants.
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::XffValue;
    ///
    /// let boolean_value_true = XffValue::from(true);
    /// let boolean_value_false = XffValue::from(false);
    /// let string_value = XffValue::from("hello mom!");
    ///
    /// assert!(!string_value.is_false());
    /// assert!(!boolean_value_true.is_false());
    /// assert!(boolean_value_false.is_false());
    /// ```
    pub fn is_false(&self) -> bool {
        matches!(self, XffValue::Boolean(false))
    }

    /// Checks if the value is null, returns `true` if it is.
    /// Returns `false` for all other variants.
    ///
    /// # Example
    /// ```rust
    /// use nabu::xff::value::XffValue;
    ///
    /// let null_value = XffValue::Null;
    /// let string_value = XffValue::from("hello mom!");
    ///
    /// assert!(!string_value.is_null());
    /// assert!(null_value.is_null());
    /// ```
    pub fn is_null(&self) -> bool {
        matches!(self, XffValue::Null)
    }
    
}

// -----------------------------------------------------------
//                     From implementations
// -----------------------------------------------------------

impl From<CommandCharacter> for XffValue {
    fn from(c: CommandCharacter) -> Self {
        XffValue::CommandCharacter(c)
    }
}

impl From<Data> for XffValue {
    fn from(c: Data) -> Self {
        XffValue::Data(c)
    }
}

impl Default for XffValue {
    fn default() -> Self {
        XffValue::String(String::new())
    }
}

impl From<bool> for XffValue {
    fn from(c: bool) -> Self {
        XffValue::Boolean(c)
    }
}

impl From<BTreeMap<String, XffValue>> for XffValue {
    fn from(c: BTreeMap<String, XffValue>) -> Self {
        XffValue::Object(c)
    }
}

impl From<HashMap<String, XffValue>> for XffValue {
    fn from(c: HashMap<String, XffValue>) -> Self {
        XffValue::Object(c.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
    }
}

impl From<Vec<XffValue>> for XffValue {
    fn from(c: Vec<XffValue>) -> Self {
        XffValue::Array(c)
    }
}

impl From<Vec<u8>> for XffValue {
    fn from(c: Vec<u8>) -> Self {
        XffValue::Data(Data::from(c))
    }
}

impl From<Vec<CommandCharacter>> for XffValue {
    fn from(c: Vec<CommandCharacter>) -> Self {
        XffValue::ArrayCmdChar(c)
    }
}

impl<S> From<(S, u8)> for XffValue where S: Into<String> {
    fn from(c: (S, u8)) -> Self {
        match c.1 {
            0 => {
                let string = c.0.into();
                let check_usize = &string.parse::<usize>();
                if check_usize.is_ok() {
                    XffValue::Number(Number::from(check_usize.as_ref().unwrap()))
                } else {
                    let check_isize = &string.parse::<isize>();
                    if check_isize.is_ok() {
                        XffValue::Number(Number::from(check_isize.as_ref().unwrap()))
                    } else {
                        let check_float = &string.parse::<f64>();
                        if check_float.is_ok() {
                            XffValue::Number(Number::from(check_float.as_ref().unwrap()))
                        } else {
                            XffValue::String(string)
                        }
                    }
                }
            },
            1 => XffValue::String(c.0.into()),
            _ => unreachable!(),
        }
    }
}

impl From<String> for XffValue {
    fn from(c: String) -> Self {
        XffValue::String(c)
    }
}

impl From<&str> for XffValue {
    fn from(c: &str) -> Self {
        XffValue::String(c.to_string())
    }
}

impl From<usize> for XffValue {
    fn from(c: usize) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<isize> for XffValue {
    fn from(c: isize) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<f64> for XffValue {
    fn from(c: f64) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<u64> for XffValue {
    fn from(c: u64) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<i64> for XffValue {
    fn from(c: i64) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<f32> for XffValue {
    fn from(c: f32) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<u32> for XffValue {
    fn from(c: u32) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<i32> for XffValue {
    fn from(c: i32) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<u16> for XffValue {
    fn from(c: u16) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<i16> for XffValue {
    fn from(c: i16) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<u8> for XffValue {
    fn from(c: u8) -> Self {
        XffValue::Number(Number::from(c))
    }
}

impl From<i8> for XffValue {
    fn from(c: i8) -> Self {
        XffValue::Number(Number::from(c))
    }
}

