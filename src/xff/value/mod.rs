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

enum CommandCharacter {}
