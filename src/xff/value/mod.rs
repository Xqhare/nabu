#[derive(Debug, Clone, PartialEq)]
pub enum XffValue {
    String(String),
    Number(Number),
    Data(Data),
    CommandCharacter(CommandCharacter),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Unsigned(usize),
    Integer(isize),
    Float(f64),
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
    Null,
    StartOfHeading,
    StartOfText,
    EndOfText,
    EndOfTransmission,
    Enquiry,
    Acknowledge,
    Bell,
    Backspace,
    HorizontalTab,
    LineFeed,
    VerticalTab,
    FormFeed,
    CarriageReturn,
    ShiftOut,
    ShiftIn,
    DataLinkEscape,
    DeviceControl1,
    DeviceControl2,
    DeviceControl3,
    DeviceControl4,
    NegativeAcknowledge,
    SynchronousIdle,
    EndOfTransmitBlock,
    Cancel,
    EndOfMedium,
    Substitute,
    Escape,
    FileSeparator,
    GroupSeparator,
    RecordSeparator,
    UnitSeparator,
    Space,
    Delete,
    NonBreakingSpace,
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
