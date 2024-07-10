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
    G9
}

/// Register references to processor registers. Registers are encoded with 4 bits. 
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Register {
    /// General purpose register for finalizing results or final computation.
    Accumulator,
    Pointer(Pointer),
    SideInput(SideInput),
    GeneralPurpose(GeneralPurpose)
}

/// The encoded register code used was invalid. 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InvalidRegisterCodeError;

impl Register {
    pub const ACCUMULATOR: u8 = 0;
    
    pub const STACK_POINTER: u8 = Self::ACCUMULATOR + 1;
    pub const BASE_POINTER: u8 = Self::STACK_POINTER + 1;
    
    pub const FIRST_SIDE_INPUT: u8 = Self::BASE_POINTER + 1;
    pub const SECOND_SIDE_INPUT: u8 = Self::FIRST_SIDE_INPUT + 1;
    
    pub const GENERAL_PURPOSE_0: u8 = Self::SECOND_SIDE_INPUT + 1;
    pub const GENERAL_PURPOSE_1: u8 = Self::GENERAL_PURPOSE_0 + 1;
    pub const GENERAL_PURPOSE_2: u8 = Self::GENERAL_PURPOSE_1 + 1;
    pub const GENERAL_PURPOSE_3: u8 = Self::GENERAL_PURPOSE_2 + 1;
    pub const GENERAL_PURPOSE_4: u8 = Self::GENERAL_PURPOSE_3 + 1;
    pub const GENERAL_PURPOSE_5: u8 = Self::GENERAL_PURPOSE_4 + 1;
    pub const GENERAL_PURPOSE_6: u8 = Self::GENERAL_PURPOSE_5 + 1;
    pub const GENERAL_PURPOSE_7: u8 = Self::GENERAL_PURPOSE_6 + 1;
    pub const GENERAL_PURPOSE_8: u8 = Self::GENERAL_PURPOSE_7 + 1;
    pub const GENERAL_PURPOSE_9: u8 = Self::GENERAL_PURPOSE_8 + 1;
    
    /// Encode the register into a 4 bit code.
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
                GeneralPurpose::G9 => Self::GENERAL_PURPOSE_9
            }
        }
    }
    
    /// Decode a register code into a register. An invalid code that ranges beyond what 4 bits can represent will return
    /// [InvalidRegisterCodeError].
    /// ```
    /// use arrseq_instruction::operand::register::{GeneralPurpose, Register};
    /// 
    /// assert_eq!(Register::decode(Register::GENERAL_PURPOSE_8).unwrap(), Register::GeneralPurpose(GeneralPurpose::G8));
    /// assert_eq!(Register::decode(Register::GENERAL_PURPOSE_4).unwrap(), Register::GeneralPurpose(GeneralPurpose::G4));
    /// ```
    pub fn decode(encoded: u8) -> Result<Self, InvalidRegisterCodeError> {
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
            _ => return Err(InvalidRegisterCodeError)
        })
    }
}

// Encode a byte with 2 registers. 
// - Encode the first register in the upper half of this byte.
// - Encode the second register in the lower half of this byte.
//
// It is safe to unwrap on these methods. The register packing uses 4 bits and register codes also use 4 bits, for 
// this reason, an [InvalidRegisterCodeError] is impossible.
pub mod dual_registers_encoding {
    use crate::operand::register::Register;

    pub fn encode_first(mut encoded: u8, first: Register) -> u8 {
        encoded |= first.encode() << 4;
        encoded
    }

    pub fn decode_first(encoded: u8) -> Register {
        Register::decode(encoded >> 4).unwrap()
    }

    pub fn encode_second(mut encoded: u8, second: Register) -> u8 {
        encoded |= second.encode();
        encoded
    }

    pub fn decode_second(encoded: u8) -> Register {
        // Ensure that bits from the first register do not increase the value of the second.
        Register::decode(encoded & 0b0000_1111).unwrap()
    }

    pub fn encode(mut encoded: u8, first: Register, second: Register) -> u8 {
        encoded = encode_first(encoded, first);
        encoded = encode_second(encoded, second);
        encoded
    }

    pub fn decode(encoded: u8) -> (Register, Register) {
        (decode_first(encoded), decode_second(encoded))
    }
}