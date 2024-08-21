use proc_bitfield::{bitfield, ConvRaw};

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Format(pub u32): Debug, FromRaw, IntoRaw { pub operation: u8 [unsafe! Operation] @ 5..=8 }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, ConvRaw)]
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