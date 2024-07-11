use arrseq_memory::dynamic_number;
use crate::operand;

/// Metadata for the operand involving the size of the operands, addressing mode, and more.
///
/// Some fields are privately initiated to ensure the validity of the data.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Meta {
    /// The size of the data being referenced by the operand(s).
    pub size: dynamic_number::Size,

    /// The name of the operand in which to store the result in.
    pub result: operand::Name,

    /// This data does not control the encoder and can be used to indicate any boolean based value.
    pub custom_data: bool,

    /// The encoded code of the dynamic operand.
    dynamic_code: u8,
}

impl Meta {
    /// # Result
    /// Instance of [Self] as long as the dynamic code is valid otherwise [Err(operand::dynamic::InvalidCodeError)] is
    /// returned.
    pub fn new(size: dynamic_number::Size, result: operand::Name, custom_data: bool, dynamic_code: u8) -> Result<Self, operand::dynamic::InvalidCodeError> {
        if !operand::dynamic::Dynamic::is_valid(dynamic_code) { return Err(operand::dynamic::InvalidCodeError) }
        Ok(Self { size, result, custom_data, dynamic_code })
    }

    pub fn encode(self) -> u8 {
        let mut encoded = dynamic_number::Size::from(self.size).exponent_representation() << 6;
        encoded |= (matches!(self.result, operand::Name::Dynamic) as u8) << 5;
        encoded |= self.dynamic_code << 1;
        encoded |= self.custom_data as u8;
        encoded
    }

    /// # Result
    /// This function has no error because the dynamic code is never invalid. Valid dynamic codes are 4 bits.
    pub fn decode(encoded: u8) -> Self {
        let size = dynamic_number::Size::from_exponent_representation(encoded >> 6).unwrap();
        let result = if (encoded >> 5) & 0b0000000_1 == 1 { operand::Name::Dynamic } else { operand::Name::Register };
        let dynamic_code = encoded >> 1 & 0b000_1111_0;
        let custom_data = encoded & 0b0000000_1 == 1;
        Self { size, result, dynamic_code, custom_data }
    }

    pub fn dynamic_code(self) -> u8 {
        self.dynamic_code
    }
}