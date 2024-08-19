#[derive(Debug, Clone, PartialEq)]
/// A numeric value
/// `Number::form()` is implemented for all numeric types
pub enum Number {
    /// An unsigned integer
    Unsigned(usize),
    /// An integer
    Integer(isize),
    /// A float
    Float(f64),
}

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

impl Number {
    /// Returns the number as bytes in string (ASCII) form
    pub fn as_u8(&self) -> Vec<u8> {
        match self {
            Number::Unsigned(u) => {
                let tmp = format!("{}", u);
                tmp.into_bytes()
            }
            Number::Integer(i) => {
                let tmp = format!("{}", i);
                tmp.into_bytes()
            }
            Number::Float(f) => {
                let tmp = format!("{}", f);
                tmp.into_bytes()
            }
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Number::Unsigned(u) => format!("{}", u),
            Number::Integer(i) => format!("{}", i),
            Number::Float(f) => format!("{}", f),
        }
    }
}

