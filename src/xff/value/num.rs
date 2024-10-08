#[derive(Debug, Clone, PartialEq)]
/// A numeric value.
///
/// `Number::form()` is implemented for all numeric types
///
/// # Example
/// ```rust
/// use nabu::Number;
///
/// let num_float = Number::from(42.69);
/// let num_unsigned = Number::from(u8::from(42));
/// let num_integer = Number::from(-42);
///
/// assert!(num_float.is_float());
/// assert!(num_unsigned.is_unsigned());
/// assert!(num_integer.is_integer());
///
/// assert_eq!(num_float.into_usize(), None);
///
/// assert_eq!(num_unsigned.into_usize(), Some(42));
///
/// assert_eq!(num_integer.as_string(), "-42".to_string());
/// ```
pub enum Number {
    /// An unsigned integer
    Unsigned(usize),
    /// An integer
    Integer(isize),
    /// A float
    Float(f64),
}

// -----------------------------------------------------------
//                     General implementations
// -----------------------------------------------------------

impl Number {
    /// Returns the value as an unsigned integer if it is a `Number::Unsigned`
    /// Returns `None` for all other variants
    ///
    /// # Example
    /// ```rust
    /// use nabu::Number;
    ///
    /// let num_u = Number::from(u8::from(42));
    /// let num_f = Number::from(42.69);
    ///
    /// assert_eq!(num_u.into_usize(), Some(42));
    /// assert_eq!(num_f.into_usize(), None);
    /// ```
    pub fn into_usize(self) -> Option<usize> {
        match self {
            Number::Unsigned(u) => Some(u),
            _ => None,
        }
    }

    /// Returns the value as an integer if it is a `Number::Integer`
    /// Returns `None` for all other variants
    ///
    /// # Example
    /// ```rust
    /// use nabu::Number;
    ///
    /// let num_i = Number::from(-42);
    /// let num_f = Number::from(42.69);
    ///
    /// assert_eq!(num_i.into_isize(), Some(-42));
    /// assert_eq!(num_f.into_isize(), None);
    /// ```
    pub fn into_isize(self) -> Option<isize> {
        match self {
            Number::Integer(i) => Some(i),
            _ => None,
        }
    }

    /// Returns the value as a float if it is a `Number::Float`
    /// Returns `None` for all other variants
    ///
    /// # Example
    /// ```rust
    /// use nabu::Number;
    ///
    /// let num_f = Number::from(42.69);
    /// let num_i = Number::from(-42);
    ///
    /// assert_eq!(num_f.into_f64(), Some(42.69));
    /// assert_eq!(num_i.into_f64(), None);
    /// ```
    pub fn into_f64(self) -> Option<f64> {
        match self {
            Number::Float(f) => Some(f),
            _ => None,
        }
    }

    /// Returns `true` if the number is an unsigned integer
    /// Returns `false` for all other variants
    ///
    /// # Example
    /// ```rust
    /// use nabu::Number;
    ///
    /// let num_u = Number::from(u8::from(42));
    /// assert!(num_u.is_unsigned());
    /// ```
    pub fn is_unsigned(&self) -> bool {
        matches!(self, Number::Unsigned(_))
    }

    /// Returns `true` if the number is an integer
    /// Returns `false` for all other variants
    ///
    /// # Example
    /// ```rust
    /// use nabu::Number;
    ///
    /// let num_i = Number::from(-42);
    /// assert!(num_i.is_integer());
    /// ```
    pub fn is_integer(&self) -> bool {
        matches!(self, Number::Integer(_))
    }

    /// Returns `true` if the number is a float
    /// Returns `false` for all other variants
    ///
    /// # Example
    /// ```rust
    /// use nabu::Number;
    ///
    /// let num_f = Number::from(42.69);
    /// assert!(num_f.is_float());
    /// ```
    pub fn is_float(&self) -> bool {
        matches!(self, Number::Float(_))
    }

    /// Returns the number formatted as an ASCII string
    ///
    /// # Example
    /// ```rust
    /// use nabu::Number;
    ///
    /// let num_u = Number::from(42);
    /// assert_eq!(num_u.as_string(), "42".to_string());
    ///
    /// let num_i = Number::from(-42);
    /// assert_eq!(num_i.as_string(), "-42".to_string());
    ///
    /// let num_f = Number::from(42.69);
    /// assert_eq!(num_f.as_string(), "42.69".to_string());
    /// ```
    pub fn as_string(&self) -> String {
        match self {
            Number::Unsigned(u) => format!("{}", u),
            Number::Integer(i) => format!("{}", i),
            Number::Float(f) => format!("{}", f),
        }
    }
}

// -----------------------------------------------------------
//                     From implementations
// -----------------------------------------------------------

impl From<usize> for Number {
    fn from(c: usize) -> Self {
        Number::Unsigned(c)
    }
}

impl From<&usize> for Number {
    fn from(c: &usize) -> Self {
        Number::Unsigned(c.clone())
    }
}

impl From<u64> for Number {
    fn from(c: u64) -> Self {
        Number::Unsigned(c as usize)
    }
}

impl From<u32> for Number {
    fn from(c: u32) -> Self {
        Number::Unsigned(c as usize)
    }
}

impl From<u16> for Number {
    fn from(c: u16) -> Self {
        Number::Unsigned(c as usize)
    }
}

impl From<u8> for Number {
    fn from(c: u8) -> Self {
        Number::Unsigned(c as usize)
    }
}

impl From<isize> for Number {
    fn from(c: isize) -> Self {
        Number::Integer(c)
    }
}

impl From<&isize> for Number {
    fn from(c: &isize) -> Self {
        Number::Integer(c.clone())
    }
}

impl From<i64> for Number {
    fn from(c: i64) -> Self {
        Number::Integer(c as isize)
    }
}

impl From<i32> for Number {
    fn from(c: i32) -> Self {
        Number::Integer(c as isize)
    }
}

impl From<i16> for Number {
    fn from(c: i16) -> Self {
        Number::Integer(c as isize)
    }
}

impl From<i8> for Number {
    fn from(c: i8) -> Self {
        Number::Integer(c as isize)
    }
}

impl From<f64> for Number {
    fn from(c: f64) -> Self {
        Number::Float(c)
    }
}

impl From<&f64> for Number {
    fn from(c: &f64) -> Self {
        Number::from(c.clone())
    }
}

impl From<f32> for Number {
    fn from(c: f32) -> Self {
        Number::Float(c as f64)
    }
}

// -----------------------------------------------------------
//                     Display implementation
// -----------------------------------------------------------

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Number::Unsigned(u) => write!(f, "{}", u),
            Number::Integer(i) => write!(f, "{}", i),
            Number::Float(fl) => write!(f, "{}", fl),
        }
    }
}
