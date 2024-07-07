use instruction::operand;
use utility::ToCode;

#[repr(u16)]
pub enum Code {
    Add,
    Subtract
}

#[derive(Debug)]
pub enum Floating {
    Add(operand::Dual),
    Subtract(operand::Dual)
}

impl ToCode for Floating {
    type Code = u16;

    fn to_code(&self) -> Self::Code {
        (match self {
            Self::Add(_) => Code::Add,
            Self::Subtract(_) => Code::Subtract
        }) as Self::Code
    }
}