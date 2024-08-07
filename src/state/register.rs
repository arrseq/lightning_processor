use crate::num;

pub const REGISTER_MASK: u8 = 0x03;
pub type Register = num::MaskedU8<REGISTER_MASK>;