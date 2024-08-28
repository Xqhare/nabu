// ----------------------------------------
//          DEPRECATED - V0 FEATURE
//           needed for legacy use
// ----------------------------------------

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
