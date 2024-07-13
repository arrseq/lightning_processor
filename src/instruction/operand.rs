use std::io;
use std::io::{Read, Write};
use crate::dynamic_number;
use super::operand;
use super::operand::dynamic::Dynamic;
use super::operand::register::Register;

pub mod dynamic;
pub mod register;

/// Named of the 2 supported operands.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Name {
    /// Register only operands.
    Register,
    
    /// Dynamically addressed operand. This operand could potentially refer to one of many things.
    Dynamic
}

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
    pub fn new(size: dynamic_number::Size, result: Name, custom_data: bool, dynamic_code: u8) -> Result<Self, dynamic::InvalidCodeError> {
        if !Dynamic::is_valid(dynamic_code) { return Err(dynamic::InvalidCodeError) }
        Ok(Self { size, result, custom_data, dynamic_code })
    }

    pub fn encode(self) -> u8 {
        let mut encoded = self.size.exponent_representation() << 6;
        encoded |= (matches!(self.result, Name::Dynamic) as u8) << 5;
        encoded |= self.dynamic_code << 1;
        encoded |= self.custom_data as u8;
        encoded
    }

    /// # Result
    /// This function has no error because the dynamic code is never invalid. Valid dynamic codes are 4 bits.
    pub fn decode(encoded: u8) -> Result<Self, dynamic::InvalidCodeError> {
        let size = dynamic_number::Size::from_exponent_representation(encoded >> 6).unwrap();
        let result = if (encoded >> 5) & 0b0000000_1 == 1 { Name::Dynamic } else { Name::Register };
        let dynamic_code = (encoded & 0b000_1111_0) >> 1;
        if !Dynamic::is_valid(dynamic_code) { return Err(dynamic::InvalidCodeError); }
        let custom_data = encoded & 0b0000000_1 == 1;
        Ok(Self { size, result, dynamic_code, custom_data })
    }

    pub fn dynamic_code(self) -> u8 {
        self.dynamic_code
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Operands {
    /// The size of the data that the operands refer to.
    pub size: dynamic_number::Size,
    pub result: Name,
    pub register: Register,
    pub dynamic: Dynamic,
    pub custom_data: bool
}

#[derive(Debug)]
pub enum DecodeError {
    InvalidDynamicCode(dynamic::InvalidCodeError),
    Read(io::Error)
}

impl Operands {
    /// ```
    /// use arrseq_instruction::operand;
    /// use arrseq_instruction::operand::{Combination, Operands, RegisterAndDynamic};
    /// use arrseq_instruction::operand::dynamic::Dynamic;
    /// use arrseq_instruction::operand::register::Register;
    /// use crate::dynamic_number;
    ///
    /// let operands = Operands {
    ///     size: dynamic_number::Size::Word,
    ///     combination: Combination::RegisterAndDynamic(RegisterAndDynamic {
    ///         result: operand::Name::Register,
    ///         register: Register::Accumulator,
    ///         dynamic: Dynamic::Constant(dynamic_number::Unsigned::Word(10))
    ///     })
    /// };
    /// ```
    pub fn decode(input: &mut impl Read) -> Result<Self, DecodeError> {
        // Contains the meta and registers bytes.
        let mut buffer = [0u8; 2];
        input.read_exact(&mut buffer).map_err(DecodeError::Read)?;

        // Code after this should unwrap on things that return [Err(dynamic::InvalidCodeError)] due to this.
        let meta = Meta::decode(buffer[0]).map_err(DecodeError::InvalidDynamicCode)?;
        let registers = register::Dual::decode(buffer[1]);
        
        let dynamic = match Dynamic::requirement(meta.dynamic_code).unwrap() {
            dynamic::Requirement::Register => Dynamic::decode_register(meta.dynamic_code, registers.second),
            dynamic::Requirement::Constant(size) => Dynamic::decode_constant(meta.dynamic_code, Self::decode_constant(input, size.unwrap_or(meta.size)).map_err(DecodeError::Read)?),
            // There is no dynamic operand mode with this requirement that uses [None] for its constant size here. It is
            // acceptable to unwrap here.
            dynamic::Requirement::RegisterAndConstant(size) => {
                let calculated = dynamic::Calculated {
                    base: registers.second,
                    offset: Self::decode_constant(input, size.unwrap_or(meta.size)).map_err(DecodeError::Read)?
                };
                
                Dynamic::decode_calculated(meta.dynamic_code, calculated)
            }
        }.unwrap();
        
        Ok(Self {
            size: meta.size,
            result: meta.result,
            register: registers.first,
            dynamic,
            custom_data: meta.custom_data
        })
    }
    
    pub fn decode_constant(input: &mut impl Read, size: dynamic_number::Size) -> Result<dynamic_number::Unsigned, io::Error> {
        let mut quad_word_buffer = [0u8; dynamic_number::Size::QUAD_WORD_BYTES];
        let buffer = match size {
            dynamic_number::Size::Byte => &mut quad_word_buffer[0..1],
            dynamic_number::Size::Word => &mut quad_word_buffer[0..dynamic_number::Size::WORD_BYTES],
            dynamic_number::Size::DoubleWord => &mut quad_word_buffer[0..dynamic_number::Size::DOUBLE_WORD_BYTES],
            dynamic_number::Size::QuadWord => &mut quad_word_buffer[0..dynamic_number::Size::QUAD_WORD_BYTES]
        };
        
        input.read_exact(buffer)?;
        Ok(dynamic_number::Unsigned::from_le_bytes(buffer).unwrap())
    }
    
    pub fn encode(self, output: &mut impl Write) -> Result<(), io::Error> {
        // This will not fail because the dynamic operand is being encoded from a valid dynamic operand.
        let meta = Meta::new(self.size, self.result, self.custom_data, self.dynamic.encode()).unwrap();
        let registers = register::Dual {
            first: self.register,
            second: self.dynamic.register().unwrap_or_default()
        };
        
        let buffer = [meta.encode(), registers.encode()];
        output.write_all(&buffer)?;
        
        if let Ok(constant) = self.dynamic.constant() { Self::encode_constant(output, constant)?; }
        Ok(())
    }
    
    pub fn encode_constant(output: &mut impl Write, constant: dynamic_number::Unsigned) -> Result<(), io::Error> {
        let bytes = constant.to_le_bytes();
        let buffer = bytes.as_slice();
        output.write_all(buffer)
    }
}