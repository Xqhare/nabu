#[derive(Debug, Clone, PartialEq)]
pub enum XffValue {
    String(String),
    Number(Number),
    Data(Data),
    CommandCharacter(CommandCharacter),
    ArrayCmdChar(Vec<CommandCharacter>),
}

impl Default for XffValue {
    fn default() -> Self {
        XffValue::String(String::new())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Unsigned(usize),
    Integer(isize),
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
    pub fn as_u8(&self) -> Vec<u8> {
        match self {
            Number::Unsigned(u) => {
                let tmp = format!("{}", u);
                tmp.into_bytes()
            },
            Number::Integer(i) => {
                let tmp = format!("{}", i);
                tmp.into_bytes()
            },
            Number::Float(f) => {
                let tmp = format!("{}", f);
                tmp.into_bytes()
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Data {
    pub data: Vec<u8>,
    pub len: usize,
}

impl From<Vec<u8>> for Data {
    fn from(data: Vec<u8>) -> Self {
        Data { data: data.clone(), len: data.len() }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
