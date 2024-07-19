use thiserror::Error;

/// Pointer to the boundaries of the stack or the current stack frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pointer {
    /// Pointer to the top of the stack.
    Stack,
    /// Pointer to the start of the current stack frame.
    Base
}

/// Register that can be used by specific operations for specifying or returning extra and or specific information.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SideInput {
    First,
    Second
}

/// No purpose general purpose registers. There is no constraint on how you use these.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeneralPurpose {
    G0,
    G1,
    G2,
    G3,
    G4,
    G5,
    G6,
    G7,
    G8,
    G9,
    G10
}

/// Register references to processor registers. Registers are encoded with 4 bits. 
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Register {
    /// General purpose register for finalizing results or final computation.
    #[default]
    Accumulator,
    Pointer(Pointer),
    SideInput(SideInput),
    GeneralPurpose(GeneralPurpose)
}

/// The encoded register code used was invalid. 
#[derive(Debug, Error)]
#[error("Referring code does not map to a register")]
pub struct InvalidCodeError;

impl Register {
    pub const ACCUMULATOR: u8 = 0;
    
    pub const STACK_POINTER: u8 = 1;
    pub const BASE_POINTER: u8 = 2;
    
    pub const FIRST_SIDE_INPUT: u8 = 3;
    pub const SECOND_SIDE_INPUT: u8 = 4;
    
    pub const GENERAL_PURPOSE_0: u8 = 5;
    pub const GENERAL_PURPOSE_1: u8 = 6;
    pub const GENERAL_PURPOSE_2: u8 = 7;
    pub const GENERAL_PURPOSE_3: u8 = 8;
    pub const GENERAL_PURPOSE_4: u8 = 9;
    pub const GENERAL_PURPOSE_5: u8 = 10;
    pub const GENERAL_PURPOSE_6: u8 = 11;
    pub const GENERAL_PURPOSE_7: u8 = 12;
    pub const GENERAL_PURPOSE_8: u8 = 13;
    pub const GENERAL_PURPOSE_9: u8 = 14;
    pub const GENERAL_PURPOSE_10: u8 = 15;
    
    /// Encode the register into a 4 bit code.
    ///
    /// # Example
    /// ```
    /// use arrseq_instruction::operand::register::{GeneralPurpose, Register};
    /// 
    /// assert_eq!(Register::GENERAL_PURPOSE_8, Register::GeneralPurpose(GeneralPurpose::G8).encode());
    /// assert_eq!(Register::GENERAL_PURPOSE_4, Register::GeneralPurpose(GeneralPurpose::G4).encode());
    /// ```
    pub fn encode(self) -> u8 {
        match self {
            Self::Accumulator => Self::ACCUMULATOR,
            Self::Pointer(pointer) => match pointer {
                Pointer::Stack => Self::STACK_POINTER,
                Pointer::Base => Self::BASE_POINTER
            },
            Self::SideInput(side_input) => match side_input {
                SideInput::First => Self::FIRST_SIDE_INPUT,
                SideInput::Second => Self::SECOND_SIDE_INPUT
            },
            Self::GeneralPurpose(general_purpose) => match general_purpose {
                GeneralPurpose::G0 => Self::GENERAL_PURPOSE_0,
                GeneralPurpose::G1 => Self::GENERAL_PURPOSE_1,
                GeneralPurpose::G2 => Self::GENERAL_PURPOSE_2,
                GeneralPurpose::G3 => Self::GENERAL_PURPOSE_3,
                GeneralPurpose::G4 => Self::GENERAL_PURPOSE_4,
                GeneralPurpose::G5 => Self::GENERAL_PURPOSE_5,
                GeneralPurpose::G6 => Self::GENERAL_PURPOSE_6,
                GeneralPurpose::G7 => Self::GENERAL_PURPOSE_7,
                GeneralPurpose::G8 => Self::GENERAL_PURPOSE_8,
                GeneralPurpose::G9 => Self::GENERAL_PURPOSE_9,
                GeneralPurpose::G10 => Self::GENERAL_PURPOSE_10
            }
        }
    }
    
    /// Decode a register code into a register.
    ///
    /// # Result
    /// An invalid code that ranges beyond what 4 bits can represent will return [InvalidCodeError].
    ///
    /// # Example
    /// ```
    /// use arrseq_instruction::operand::register::{GeneralPurpose, Register};
    /// 
    /// assert_eq!(Register::decode(Register::GENERAL_PURPOSE_8).unwrap(), Register::GeneralPurpose(GeneralPurpose::G8));
    /// assert_eq!(Register::decode(Register::GENERAL_PURPOSE_4).unwrap(), Register::GeneralPurpose(GeneralPurpose::G4));
    /// ```
    pub fn decode(encoded: u8) -> Result<Self, InvalidCodeError> {
        Ok(match encoded {
            Self::ACCUMULATOR => Self::Accumulator,
            Self::STACK_POINTER => Self::Pointer(Pointer::Stack),
            Self::BASE_POINTER => Self::Pointer(Pointer::Base),
            Self::FIRST_SIDE_INPUT => Self::SideInput(SideInput::First),
            Self::SECOND_SIDE_INPUT => Self::SideInput(SideInput::Second),
            Self::GENERAL_PURPOSE_0 => Self::GeneralPurpose(GeneralPurpose::G0),
            Self::GENERAL_PURPOSE_1 => Self::GeneralPurpose(GeneralPurpose::G1),
            Self::GENERAL_PURPOSE_2 => Self::GeneralPurpose(GeneralPurpose::G2),
            Self::GENERAL_PURPOSE_3 => Self::GeneralPurpose(GeneralPurpose::G3),
            Self::GENERAL_PURPOSE_4 => Self::GeneralPurpose(GeneralPurpose::G4),
            Self::GENERAL_PURPOSE_5 => Self::GeneralPurpose(GeneralPurpose::G5),
            Self::GENERAL_PURPOSE_6 => Self::GeneralPurpose(GeneralPurpose::G6),
            Self::GENERAL_PURPOSE_7 => Self::GeneralPurpose(GeneralPurpose::G7),
            Self::GENERAL_PURPOSE_8 => Self::GeneralPurpose(GeneralPurpose::G8),
            Self::GENERAL_PURPOSE_9 => Self::GeneralPurpose(GeneralPurpose::G9),
            Self::GENERAL_PURPOSE_10 => Self::GeneralPurpose(GeneralPurpose::G10),
            _ => return Err(InvalidCodeError)
        })
    }
}

/// Representation of two registers.
///
/// # Encoding
/// The first register is encoded in the upper half of the encoded byte and the second register is encoded in the lower
/// half of the byte.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dual {
    pub first: Register,
    pub second: Register
}

impl Dual {
    /// # Result
    /// Decoding will always work because the register division involves 4 bits and a valid register code must be 4 bits.
    pub fn decode(encoded: u8) -> Self {
        let first_encoded = encoded >> 4;
        let second_encoded = encoded & 0b0000_1111;
        Self {
            first: Register::decode(first_encoded).unwrap(),
            second: Register::decode(second_encoded).unwrap()
        }
    }

    pub fn encode(self) -> u8 {
        let mut encoded = self.second.encode();
        encoded |= self.first.encode() << 4;
        encoded
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Collection {
    pub accumulator: u64,
    
    pub stack_pointer: u64,
    pub base_pointer: u64,
    
    pub first_side_input: u64,
    pub second_side_input: u64,

    pub general_purpose: [u64; 11]
}

impl Collection {
    pub fn get_mut(&mut self, register: Register) -> &mut u64 {
        match register {
            Register::Accumulator => &mut self.accumulator,
            Register::Pointer(pointer) => match pointer {
                Pointer::Stack => &mut self.stack_pointer,
                Pointer::Base => &mut self.base_pointer
            },
            Register::SideInput(side_input) => match side_input {
                SideInput::First => &mut self.first_side_input,
                SideInput::Second => &mut self.second_side_input
            },
            Register::GeneralPurpose(general_purpose) => match general_purpose {
                GeneralPurpose::G0 => &mut self.general_purpose[0],
                GeneralPurpose::G1 => &mut self.general_purpose[1],
                GeneralPurpose::G2 => &mut self.general_purpose[2],
                GeneralPurpose::G3 => &mut self.general_purpose[3],
                GeneralPurpose::G4 => &mut self.general_purpose[4],
                GeneralPurpose::G5 => &mut self.general_purpose[5],
                GeneralPurpose::G6 => &mut self.general_purpose[6],
                GeneralPurpose::G7 => &mut self.general_purpose[7],
                GeneralPurpose::G8 => &mut self.general_purpose[8],
                GeneralPurpose::G9 => &mut self.general_purpose[9],
                GeneralPurpose::G10 => &mut self.general_purpose[10],
            }
        }
    }

    pub fn get(self, register: Register) -> u64 {
        match register {
            Register::Accumulator => self.accumulator,
            Register::Pointer(pointer) => match pointer {
                Pointer::Stack => self.stack_pointer,
                Pointer::Base => self.base_pointer
            },
            Register::SideInput(side_input) => match side_input {
                SideInput::First => self.first_side_input,
                SideInput::Second => self.second_side_input
            },
            Register::GeneralPurpose(general_purpose) => match general_purpose {
                GeneralPurpose::G0 => self.general_purpose[0],
                GeneralPurpose::G1 => self.general_purpose[1],
                GeneralPurpose::G2 => self.general_purpose[2],
                GeneralPurpose::G3 => self.general_purpose[3],
                GeneralPurpose::G4 => self.general_purpose[4],
                GeneralPurpose::G5 => self.general_purpose[5],
                GeneralPurpose::G6 => self.general_purpose[6],
                GeneralPurpose::G7 => self.general_purpose[7],
                GeneralPurpose::G8 => self.general_purpose[8],
                GeneralPurpose::G9 => self.general_purpose[9],
                GeneralPurpose::G10 => self.general_purpose[10],
            }
        }
    }
}