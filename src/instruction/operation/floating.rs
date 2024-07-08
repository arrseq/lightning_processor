use instruction::operand;
use instruction::operand::GetConfiguration;
use utility::ToCode;

#[repr(u16)]
pub enum Code {
    Add,
    Subtract
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