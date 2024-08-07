use crate::num;

pub const REGISTER_MASK: u8 = 0x0F;
pub type Register = num::MaskedU8<REGISTER_MASK>;