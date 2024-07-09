use strum_macros::FromRepr;
use instruction::operand;
use instruction::operand::{GetConfiguration, GetCodeConfiguration, SizedDual};
use instruction::operation::basic::Basic;
use utility::ToCode;

#[derive(Debug, Clone, Copy, FromRepr)]
#[repr(u16)]
pub enum Code {
    Add,
    Subtract
}

impl GetCodeConfiguration for Code {
    fn get_code_configuration(&self) -> Option<operand::ConfigurationCode> {
        Some(match self {
            Self::Add
                | Self::Subtract => operand::ConfigurationCode::Dual
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Floating {
    Add(operand::SizedDual),
    Subtract(operand::SizedDual)
}

impl GetConfiguration for Floating {
    fn get_configuration(&self) -> Option<operand::Configuration> {
        Some(match self {
            Self::Add(x)
                | Self::Subtract(x) => operand::Configuration::Dual(*x)
        })
    }
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

impl Floating {
    pub fn from_sized_dual(operation: Code, operands: SizedDual) -> Option<Self> {
        Some(match operation {
            Code::Add => Self::Add(operands),
            Code::Subtract => Self::Subtract(operands)
        })
    }
}