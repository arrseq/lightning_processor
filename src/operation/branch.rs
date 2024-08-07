use crate::num;
use crate::state::flag::Flag;
use crate::state::register::Register;

/// Hint to suggest whether the branch might be taken.
pub type Hint = Option<bool>;

pub const IMMEDIATE_MASK: u32 = 0x3FFFF;
pub type Immediate = num::MaskedU32<IMMEDIATE_MASK>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Absolute,
    Relative
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Address {
    Immediate {
        mode: Mode,
        immediate: Immediate },
    Register {
        mode: Mode,
        register: Register }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Condition(pub Option<Flag>);

pub const CONDITION_MASK: u8 = 0x07;
pub type ConditionCode = num::MaskedU8<CONDITION_MASK>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConditionMapping {
    pub code: ConditionCode,
    pub variant: Condition 
}

impl Condition {
    pub const MAPPINGS: [Condition; 7] = [
        Condition(None                  ),
        Condition(Some(Flag::Negative  )),
        Condition(Some(Flag::Zero      )),
        Condition(Some(Flag::Overflow  )),
        Condition(Some(Flag::Regrouping)),
        Condition(Some(Flag::Parity    )),
        Condition(Some(Flag::Auxiliary ))
    ];
    
    pub fn from_code(code: ConditionCode) -> Option<Self> {
        Self::MAPPINGS.get(code.get() as usize).copied()
    }
    
    pub fn to_code(self) -> ConditionCode {
        ConditionCode::new(
            Self::MAPPINGS
                .iter()
                .position(|&mapping| mapping == self)
                .unwrap() as u8,
        )
    }
}