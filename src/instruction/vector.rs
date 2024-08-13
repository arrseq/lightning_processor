use crate::instruction::{OperandCode};
use crate::num::MaskedU8;

pub type ComponentCode = MaskedU8<0x3>;
pub const SIZE: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VectorComponentMapping {
    operand: OperandCode,
    components: [ComponentCode; 2]
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VectorComponentFlags {
    operand: OperandCode,
    negate: bool,
    zero: bool
}