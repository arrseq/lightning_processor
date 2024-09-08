use proc_bitfield::{bitfield};

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Format(pub u32): Debug, FromRaw, IntoRaw { pub operation: u8 [unsafe! Operation] @ 5..=8 }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(u8)]
pub enum Operation {
    #[default]
    Wait,
    Interrupt,
    End,
    EndInterrupt,
    Unlock,
    Yield,
    UnYield,
    ClearRegisters,
    StackRegisters,
    UnStackRegisters
}

impl From<u8> for Operation {
    fn from(code: u8) -> Self {
        match code {
            0 => Self::Wait,
            1 => Self::Interrupt,
            2 => Self::End,
            3 => Self::EndInterrupt,
            4 => Self::Unlock,
            5 => Self::Yield,
            6 => Self::UnYield,
            7 => Self::ClearRegisters,
            8 => Self::StackRegisters,
            9 => Self::UnStackRegisters,
            _ => Self::default()
        }
    }
}

impl From<Operation> for u8 {
    fn from(operation: Operation) -> Self {
        operation as Self
    }
}

impl Operation {
    pub const TABLE: [Operation; 10] = [
        Operation::Wait,
        Operation::Interrupt,
        Operation::End,
        Operation::EndInterrupt,
        Operation::Unlock,
        Operation::Yield,
        Operation::UnYield,
        Operation::ClearRegisters,
        Operation::StackRegisters,
        Operation::UnStackRegisters
    ];
}