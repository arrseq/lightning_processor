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
pub enum Sized {
    Byte(u8),
    Word(u16)
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

impl Operation {
    /// Convert the operation to a code, then store it in a [Sized] type corresponding to what value [Size] is. This 
    /// could result in the operation code losing data and start referring to a different operation. This behavior could
    /// be undefined.
    pub fn force_code_constrained(&self, size: &Size) -> Sized {
        let code = self.to_code();

        match size {
            Size::Byte => Sized::Byte(code as u8),
            Size::Word => Sized::Word(code)
        }
    }
    
    /// Convert the operation to a code and use the smallest data type that can represent that operation.
    pub fn to_smallest_code(&self) -> Sized {
        let code = self.to_code();

        if code > u8::MAX as u16 { return Sized::Word(code); }
        Sized::Byte(code as u8)
    }
}