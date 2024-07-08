use instruction::operand;
use instruction::operand::GetConfiguration;
use instruction::operation::basic::Basic;
use instruction::operation::floating::Floating;
use number::low::{LowNumber, LowSize};
use utility::ToCode;

pub mod basic;
pub mod floating;

#[derive(Debug, Clone, Copy)]
pub enum Extension {
    Basic,
    Floating
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Basic(Basic),
    Floating(Floating)
}

impl From<&Operation> for Extension {
    fn from(value: &Operation) -> Self {
        match value {
            Operation::Basic(_) => Extension::Basic,
            Operation::Floating(_) => Extension::Floating
        }
    }
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

impl GetConfiguration for Operation {
    fn get_configuration(&self) -> Option<operand::Configuration> {
        match self {
            Self::Basic(x) => x.get_configuration(),
            Self::Floating(x) => x.get_configuration()
        }
    }
}

impl Operation {
    /// Convert the operation to a code, then store it in a [LowNumber] type corresponding to what value [LowSize] is. This
    /// could result in the operation code losing data and start referring to a different operation. This behavior could
    /// be undefined.
    pub fn force_code_constrained(&self, size: &LowSize) -> LowNumber {
        let code = self.to_code();

        match size {
            LowSize::Byte => LowNumber::Byte(code as u8),
            LowSize::Word => LowNumber::Word(code)
        }
    }

    /// Convert the operation to a code and use the smallest data type that can represent that operation.
    pub fn to_smallest_code(&self) -> LowNumber {
        let code = self.to_code();

        if code > u8::MAX as u16 { return LowNumber::Word(code); }
        LowNumber::Byte(code as u8)
    }
}