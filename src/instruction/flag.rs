use crate::num::MaskedU8;

pub type Code = MaskedU8<0x07>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Flag {
    Zero,
    Negative,
    Overflow,
    Regrouping,
    Parity
}