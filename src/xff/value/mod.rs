#[derive(Debug, Clone, PartialEq)]
/// An enum for the different types of XFF values
/// All variants except `ArrayCmdChar` are represented in the XFF format
/// `ArrayCmdChar` is a list of `CommandCharacter`s and seldom used in writing XFF files, but never in reading them
pub enum XffValue {
    /// A string value
    String(String),
    /// A numeric value
    Number(Number),
    /// A data value, holding a `Data` struct
    Data(Data),
    /// An array of command characters
    /// A command character is represented by the `CommandCharacter` enum
    CommandCharacter(CommandCharacter),
    /// An array of `CommandCharacter`s
    ArrayCmdChar(Vec<CommandCharacter>),
}

impl XffValue {
    pub fn as_string(&self) -> Option<String> {
        match self {
            XffValue::String(s) => Some(s.clone()),
            XffValue::Number(n) => Some(n.as_string()),
            _ => None,
        }
    }
}

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

impl From<String> for XffValue {
    fn from(c: String) -> Self {
        let check_usize = c.parse::<usize>();
        if check_usize.is_ok() {
            XffValue::Number(Number::from(check_usize.unwrap()))
        } else {
            let check_isize = c.parse::<isize>();
            if check_isize.is_ok() {
                XffValue::Number(Number::from(check_isize.unwrap()))
            } else {
                let check_float = c.parse::<f64>();
                if check_float.is_ok() {
                    XffValue::Number(Number::from(check_float.unwrap()))
                } else {
                    XffValue::String(c)
                }
            }
        }
    }
}

impl From<&str> for XffValue {
    fn from(c: &str) -> Self {
        let check_usize = c.parse::<usize>();
        if check_usize.is_ok() {
            XffValue::Number(Number::from(check_usize.unwrap()))
        } else {
            let check_isize = c.parse::<isize>();
            if check_isize.is_ok() {
                XffValue::Number(Number::from(check_isize.unwrap()))
            } else {
                let check_float = c.parse::<f64>();
                if check_float.is_ok() {
                    XffValue::Number(Number::from(check_float.unwrap()))
                } else {
                    XffValue::String(c.to_string())
                }
            }
        }
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

#[derive(Debug, Clone, PartialEq)]
/// Represents all command characters
///
/// Can be converted from `u8` using the `From` trait
pub enum CommandCharacter {
    /// Also known as NULL or NONE
    Null,
    /// Start of Heading
    StartOfHeading,
    /// Start of Text
    StartOfText,
    /// End of Text
    EndOfText,
    /// End of Transmission
    EndOfTransmission,
    /// Enquiry
    Enquiry,
    /// Acknowledge
    Acknowledge,
    /// Bell or Alert
    Bell,
    /// Backspace
    Backspace,
    /// Horizontal Tab or Tab
    HorizontalTab,
    /// Line Feed or New Line
    LineFeed,
    /// Vertical Tab
    VerticalTab,
    /// Form Feed
    FormFeed,
    /// Carriage Return
    CarriageReturn,
    /// Shift Out
    ShiftOut,
    /// Shift In
    ShiftIn,
    /// Data Link Escape
    DataLinkEscape,
    /// Device Control 1
    DeviceControl1,
    /// Device Control 2
    DeviceControl2,
    /// Device Control 3
    DeviceControl3,
    /// Device Control 4
    DeviceControl4,
    /// Negative Acknowledge
    NegativeAcknowledge,
    /// Synchronous Idle
    SynchronousIdle,
    /// End of Transmission Block
    EndOfTransmitBlock,
    /// Cancel
    Cancel,
    /// End of Medium
    EndOfMedium,
    /// Substitute
    Substitute,
    /// Escape
    Escape,
    /// File Separator
    FileSeparator,
    /// Group Separator
    GroupSeparator,
    /// Record Separator
    RecordSeparator,
    /// Unit Separator
    UnitSeparator,
    /// Space
    Space,
    /// Delete
    Delete,
    /// Non-Breaking Space or NBSP, no new lines allowed
    NonBreakingSpace,
    /// Soft Hyphen, no new lines allowed
    SoftHyphen,
}

impl CommandCharacter {
    /// Takes in a u8 and returns the corresponding command character
    /// If no valid command character is found, returns `CommandCharacter::Null`
    pub fn from_u8(c: u8) -> Self {
        match c {
            0 => CommandCharacter::Null,
            1 => CommandCharacter::StartOfHeading,
            2 => CommandCharacter::StartOfText,
            3 => CommandCharacter::EndOfText,
            4 => CommandCharacter::EndOfTransmission,
            5 => CommandCharacter::Enquiry,
            6 => CommandCharacter::Acknowledge,
            7 => CommandCharacter::Bell,
            8 => CommandCharacter::Backspace,
            9 => CommandCharacter::HorizontalTab,
            10 => CommandCharacter::LineFeed,
            11 => CommandCharacter::VerticalTab,
            12 => CommandCharacter::FormFeed,
            13 => CommandCharacter::CarriageReturn,
            14 => CommandCharacter::ShiftOut,
            15 => CommandCharacter::ShiftIn,
            16 => CommandCharacter::DataLinkEscape,
            17 => CommandCharacter::DeviceControl1,
            18 => CommandCharacter::DeviceControl2,
            19 => CommandCharacter::DeviceControl3,
            20 => CommandCharacter::DeviceControl4,
            21 => CommandCharacter::NegativeAcknowledge,
            22 => CommandCharacter::SynchronousIdle,
            23 => CommandCharacter::EndOfTransmitBlock,
            24 => CommandCharacter::Cancel,
            25 => CommandCharacter::EndOfMedium,
            26 => CommandCharacter::Substitute,
            27 => CommandCharacter::Escape,
            28 => CommandCharacter::FileSeparator,
            29 => CommandCharacter::GroupSeparator,
            30 => CommandCharacter::RecordSeparator,
            31 => CommandCharacter::UnitSeparator,
            32 => CommandCharacter::Space,
            127 => CommandCharacter::Delete,
            160 => CommandCharacter::NonBreakingSpace,
            173 => CommandCharacter::SoftHyphen,
            // Any invalid bytes will be treated as null
            _ => CommandCharacter::Null,
        }
    }
    /// Takes in a u8 and returns the corresponding command character
    /// If no valid command character is found, returns `none`
    pub fn from_u8_checked(c: u8) -> Option<Self> {
        match c {
            0 => Some(CommandCharacter::Null),
            1 => Some(CommandCharacter::StartOfHeading),
            2 => Some(CommandCharacter::StartOfText),
            3 => Some(CommandCharacter::EndOfText),
            4 => Some(CommandCharacter::EndOfTransmission),
            5 => Some(CommandCharacter::Enquiry),
            6 => Some(CommandCharacter::Acknowledge),
            7 => Some(CommandCharacter::Bell),
            8 => Some(CommandCharacter::Backspace),
            9 => Some(CommandCharacter::HorizontalTab),
            10 => Some(CommandCharacter::LineFeed),
            11 => Some(CommandCharacter::VerticalTab),
            12 => Some(CommandCharacter::FormFeed),
            13 => Some(CommandCharacter::CarriageReturn),
            14 => Some(CommandCharacter::ShiftOut),
            15 => Some(CommandCharacter::ShiftIn),
            16 => Some(CommandCharacter::DataLinkEscape),
            17 => Some(CommandCharacter::DeviceControl1),
            18 => Some(CommandCharacter::DeviceControl2),
            19 => Some(CommandCharacter::DeviceControl3),
            20 => Some(CommandCharacter::DeviceControl4),
            21 => Some(CommandCharacter::NegativeAcknowledge),
            22 => Some(CommandCharacter::SynchronousIdle),
            23 => Some(CommandCharacter::EndOfTransmitBlock),
            24 => Some(CommandCharacter::Cancel),
            25 => Some(CommandCharacter::EndOfMedium),
            26 => Some(CommandCharacter::Substitute),
            27 => Some(CommandCharacter::Escape),
            28 => Some(CommandCharacter::FileSeparator),
            29 => Some(CommandCharacter::GroupSeparator),
            30 => Some(CommandCharacter::RecordSeparator),
            31 => Some(CommandCharacter::UnitSeparator),
            32 => Some(CommandCharacter::Space),
            127 => Some(CommandCharacter::Delete),
            160 => Some(CommandCharacter::NonBreakingSpace),
            173 => Some(CommandCharacter::SoftHyphen),
            _ => None,
        }
    }
    /// Returns the command character as bytes in ASCII form
    pub fn as_u8(&self) -> u8 {
        match self {
            CommandCharacter::Null => 0,
            CommandCharacter::StartOfHeading => 1,
            CommandCharacter::StartOfText => 2,
            CommandCharacter::EndOfText => 3,
            CommandCharacter::EndOfTransmission => 4,
            CommandCharacter::Enquiry => 5,
            CommandCharacter::Acknowledge => 6,
            CommandCharacter::Bell => 7,
            CommandCharacter::Backspace => 8,
            CommandCharacter::HorizontalTab => 9,
            CommandCharacter::LineFeed => 10,
            CommandCharacter::VerticalTab => 11,
            CommandCharacter::FormFeed => 12,
            CommandCharacter::CarriageReturn => 13,
            CommandCharacter::ShiftOut => 14,
            CommandCharacter::ShiftIn => 15,
            CommandCharacter::DataLinkEscape => 16,
            CommandCharacter::DeviceControl1 => 17,
            CommandCharacter::DeviceControl2 => 18,
            CommandCharacter::DeviceControl3 => 19,
            CommandCharacter::DeviceControl4 => 20,
            CommandCharacter::NegativeAcknowledge => 21,
            CommandCharacter::SynchronousIdle => 22,
            CommandCharacter::EndOfTransmitBlock => 23,
            CommandCharacter::Cancel => 24,
            CommandCharacter::EndOfMedium => 25,
            CommandCharacter::Substitute => 26,
            CommandCharacter::Escape => 27,
            CommandCharacter::FileSeparator => 28,
            CommandCharacter::GroupSeparator => 29,
            CommandCharacter::RecordSeparator => 30,
            CommandCharacter::UnitSeparator => 31,
            CommandCharacter::Space => 32,
            CommandCharacter::Delete => 127,
            CommandCharacter::NonBreakingSpace => 160,
            CommandCharacter::SoftHyphen => 173,
        }
    }
}

impl From<u8> for CommandCharacter {
    fn from(c: u8) -> Self {
        match c {
            0 => CommandCharacter::Null,
            1 => CommandCharacter::StartOfHeading,
            2 => CommandCharacter::StartOfText,
            3 => CommandCharacter::EndOfText,
            4 => CommandCharacter::EndOfTransmission,
            5 => CommandCharacter::Enquiry,
            6 => CommandCharacter::Acknowledge,
            7 => CommandCharacter::Bell,
            8 => CommandCharacter::Backspace,
            9 => CommandCharacter::HorizontalTab,
            10 => CommandCharacter::LineFeed,
            11 => CommandCharacter::VerticalTab,
            12 => CommandCharacter::FormFeed,
            13 => CommandCharacter::CarriageReturn,
            14 => CommandCharacter::ShiftOut,
            15 => CommandCharacter::ShiftIn,
            16 => CommandCharacter::DataLinkEscape,
            17 => CommandCharacter::DeviceControl1,
            18 => CommandCharacter::DeviceControl2,
            19 => CommandCharacter::DeviceControl3,
            20 => CommandCharacter::DeviceControl4,
            21 => CommandCharacter::NegativeAcknowledge,
            22 => CommandCharacter::SynchronousIdle,
            23 => CommandCharacter::EndOfTransmitBlock,
            24 => CommandCharacter::Cancel,
            25 => CommandCharacter::EndOfMedium,
            26 => CommandCharacter::Substitute,
            27 => CommandCharacter::Escape,
            28 => CommandCharacter::FileSeparator,
            29 => CommandCharacter::GroupSeparator,
            30 => CommandCharacter::RecordSeparator,
            31 => CommandCharacter::UnitSeparator,
            32 => CommandCharacter::Space,
            127 => CommandCharacter::Delete,
            160 => CommandCharacter::NonBreakingSpace,
            173 => CommandCharacter::SoftHyphen,
            // Any invalid bytes will be treated as null
            _ => CommandCharacter::Null,
        }
    }
}
