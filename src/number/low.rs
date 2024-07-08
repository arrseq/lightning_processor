#[derive(Debug, Clone, Copy)]
pub enum LowSize {
    Byte,
    Word
}

#[derive(Debug, Clone, Copy)]
pub enum LowNumber {
    Byte(u8),
    Word(u16)
}

impl From<&LowNumber> for LowSize {
    fn from(value: &LowNumber) -> Self {
        match value {
            LowNumber::Byte(_) => Self::Byte,
            LowNumber::Word(_) => Self::Word
        }
    }
}