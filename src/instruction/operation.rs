use instruction::operation::basic::Basic;
use instruction::operation::floating::Floating;
use utility::ToCode;

pub mod basic;
pub mod floating;

#[derive(Debug)]
pub enum Size {
    Byte,
    Word
}

#[derive(Debug)]
pub enum Extension {
    Basic,
    Floating
}

#[derive(Debug)]
pub enum Operation {
    Basic(Basic),
    Floating(Floating)
}

impl ToCode for Operation {
    type Code = u16;

    fn to_code(&self) -> Self::Code {
        match self {
            Self::Basic(x) => x.to_code(),
            Self::Floating(x) => x.to_code()
        }
    }
}