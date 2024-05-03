use crate::utility::{Bits, Byte};

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum Register {
    Void,
    StackPointer,
    BasePointer,
    Accumulator,
    RegisterA, RegisterB,
    First,     Second,
}

impl Register {
    pub fn from_pointer(pointer: u8) -> Result<Register, ()> {
        match pointer {
            0 => Ok(Self::Void),
            1 => Ok(Self::StackPointer),
            2 => Ok(Self::BasePointer),
            3 => Ok(Self::Accumulator),
            4 => Ok(Self::RegisterA),
            5 => Ok(Self::RegisterB),
            6 => Ok(Self::First),
            7 => Ok(Self::Second),
            _ => Err(())
        }
    }
}

pub struct Operands {
    pub register_a: Register,
    pub register_b: Register
}

impl Operands {
    pub fn into_byte(&self) -> u8 {
        let a_bits = (self.register_a.clone() as u8).into_bits();
        let b_bits = (self.register_b.clone() as u8).into_bits();

        let byte_bits: [bool; 8] = [
            a_bits[4], a_bits[5], a_bits[6], a_bits[7], 
            b_bits[4], b_bits[5], b_bits[6], b_bits[7]
        ];

        byte_bits.into_byte()
    }
}