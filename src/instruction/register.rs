use utility::{FromCode, MaxWithBits};
use crate::utility::{ToCode, TryCoded, TryFromCode};

/// There are 15 different registers supported.
pub const INDEX_BITS: u8 = 4;
pub const MAX_INDEX: usize = (INDEX_BITS as usize).max_with_bits().unwrap();

/// A valid register target. 
#[derive(Debug, Clone)] pub struct Register(u8);

// region: Validity
impl TryFromCode for Register {
    type Code = u8;

    fn try_from_code(code: Self::Code) -> Option<Self> {
        if code > MAX_INDEX as u8 { return None }
        Some(Self(code))
    }
}

impl ToCode for Register {
    type Code = u8;

    fn to_code(&self) -> Self::Code { self.0 }
}

impl TryCoded for Register {}


impl FromCode for Register {
    type Code = u8;

    /// Try to perform this operation even with an invalid code. Only the least most significant bits will be used. The 
    /// number of bits read is determined by [INDEX_BITS].
    /// ```
    /// use atln_processor::instruction::register::Register;
    /// use atln_processor::utility::{FromCode, ToCode};
    ///
    /// assert_eq!(Register::from_code(0b11111111).to_code(), 0b0000_1111);
    fn from_code(code: Self::Code) -> Self {
        Self(code & MAX_INDEX as u8)
    }
}
// endregion