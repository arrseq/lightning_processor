use crate::utility::{Bits, Byte};

pub const STACK_POINTER: u8  = 0;
pub const BASE_POINTER:  u8  = 1;
pub const ACCUMULATOR:   u8  = 2;
pub const REGISTER_A:    u8  = 3;
pub const REGISTER_B:    u8  = 4;
pub const FIRST:         u8  = 5;
pub const SECOND:        u8  = 6;

pub const REGISTER_PARAM_MAX: u8 = 15;

pub struct Operands {
    register_a: u8,
    register_b: u8
}

pub enum Selector {
    A,
    B
}

impl Operands {
    /// Fails if either one of the sides are too large. 0-15 range.
    pub fn new(a: u8, b: u8) -> Result<Self, Selector> {
        if a > REGISTER_PARAM_MAX {
            return Err(Selector::A);
        }

        if b > REGISTER_PARAM_MAX {
            return Err(Selector::B);
        }

        Ok(Self {
            register_a: a,
            register_b: b
        })
    }

    pub fn into_byte(&self) -> u8 {
        let a_bits = self.register_a.into_bits();
        let b_bits = self.register_b.into_bits();

        let byte_bits: [bool; 8] = [
            a_bits[4], a_bits[5], a_bits[6], a_bits[7], 
            b_bits[4], b_bits[5], b_bits[6], b_bits[7]
        ];

        byte_bits.into_byte()
    }
}

#[test]
fn operand_into_bytes() {
    let operands = Operands {
        register_a: 10,
        register_b: 15
    };

    // The first for bits represent 10 for register A and
    // the last 4 represent 16 for register B.
    let correct = 0b10101111;

    assert_eq!(operands.into_byte(), correct);
}

#[derive(Debug, Default, Clone)]
pub struct File {
    pub stack_pointer: u64,
    pub base_pointer:  u64,
    pub accumulator:   u64,
    pub register_a:    u64,
    pub register_b:    u64,
    pub first:         u64,
    pub second:        u64
}

impl File {
    pub fn set_at(&mut self, identifier: u8, value: u64) -> Result<(), ()> {
        match identifier {
            STACK_POINTER => self.stack_pointer = value,
            BASE_POINTER  => self.base_pointer  = value,
            ACCUMULATOR   => self.accumulator   = value,
            REGISTER_A    => self.register_a    = value,
            REGISTER_B    => self.register_b    = value,
            FIRST         => self.first         = value,
            SECOND        => self.second        = value,
            _ => return Err(())
        };

        Ok(())
    }

    pub fn at_identifier(&self, identifier: u8) -> Option<u64> {
        Some(match identifier {
            STACK_POINTER => self.stack_pointer,
            BASE_POINTER  => self.base_pointer,
            ACCUMULATOR   => self.accumulator,
            REGISTER_A    => self.register_a,
            REGISTER_B    => self.register_b,
            FIRST         => self.first,
            SECOND        => self.second,
            _ => return None
        })
    }
}

#[derive(Default, Debug, Clone)]
pub enum RegisterPresence {
    #[default]
    None,
    Ab,
    A,
}

impl RegisterPresence {
    pub fn from(a: bool, b: bool) -> RegisterPresence {
        if a && b {
            RegisterPresence::Ab
        } else if a ^ b {
            RegisterPresence::A
        } else {
            RegisterPresence::None
        }
    }

    pub fn get_bytes_count(&self) -> u8 {
        match self {
            Self::None => 0,
            _ => 1
        }
    }
}