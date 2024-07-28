pub mod encoding;

use crate::math::dynamic_number::{Unsigned, Signed};

/// Mode for addressing with only the base register.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BaseAddressing {
    /// Reference something at a base address.
    Base,
    /// Reference something at a base address with an offset.
    Offsetted { offset: Unsigned }
}

/// More complex addressing that is for arrays.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArrayAddressing {
    /// Reference an array.
    Array,
    /// Reference an array that is at an offset.
    Offsetted
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComplexAddressing {
    Base            { mode: BaseAddressing             },
    ArrayAddressing { mode: ArrayAddressing, index: u8 }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComplexAddressingFieldRequirements {
    pub specifier_code: u8,
    pub requires_offset: bool,
    pub requires_index_register: bool
}

impl ComplexAddressing {
    pub const BASE_CODE: u8 = 0;
    pub const BASE_PLUS_OFFSET_CODE: u8 = 1;
    pub const ARRAY_CODE: u8 = 2;
    pub const OFFSETTED_ARRAY_CODE: u8 = 3;
    
    pub const BASE            : ComplexAddressingFieldRequirements = ComplexAddressingFieldRequirements { specifier_code: Self::BASE_CODE,             requires_offset: false, requires_index_register: false };
    pub const BASE_PLUS_OFFSET: ComplexAddressingFieldRequirements = ComplexAddressingFieldRequirements { specifier_code: Self::BASE_PLUS_OFFSET_CODE, requires_offset: true,  requires_index_register: false };
    pub const ARRAY           : ComplexAddressingFieldRequirements = ComplexAddressingFieldRequirements { specifier_code: Self::ARRAY_CODE,            requires_offset: true,  requires_index_register: true  };
    pub const OFFSETTED_ARRAY : ComplexAddressingFieldRequirements = ComplexAddressingFieldRequirements { specifier_code: Self::OFFSETTED_ARRAY_CODE,  requires_offset: true,  requires_index_register: true  };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConstantAddressing {
    /// Constant as value.
    Value,
    /// Relative to the current instruction pointer with an offset.
    Relative
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddressingMode {
    /// Retrieve the value from a register.
    Register { register: u8                               },
    /// Addressing mode that relies on the constant.
    Constant { mode: ConstantAddressing, constant: Signed },
    /// Address the value from memory.
    Complex  { mode: ComplexAddressing,  base: u8         }
}
