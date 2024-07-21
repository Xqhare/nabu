pub enum XffValue {
    String(String),
    Number(Number),
    Data(Data),
    CommandCharacter(CommandCharacter),
}

pub enum Number {
    Unsigned(usize),
    Integer(isize),
    Float(f64),
}
pub struct Data {
    pub data: Vec<u8>,
    pub len: usize,
}

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
