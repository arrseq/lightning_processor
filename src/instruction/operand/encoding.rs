#[cfg(test)]
mod test;

use std::io;
use std::io::{Read, Write};
use thiserror::Error;
use crate::instruction::operand::{AddressingMode, ArrayAddressing, BaseAddressing, ComplexAddressing, ImmediateAddressing, Operand};
use crate::math::dynamic_number::{Signed, Size, Unsigned};

#[derive(Debug, Error)]
pub(crate) enum IoError {
    #[error("Could not handle addressing byte")]
    AddressingByte,
    #[error("Could not handle immediate value")]
    ImmediateValue,
    #[error("Could not handle immediate relative offset")]
    ImmediateOffset,
    #[error("Could not handle complex addressing byte")]
    ComplexAddressingByte
}

#[derive(Debug, Error)]
#[error("Failed to access data for encoding or decoding")]
pub(crate) struct Error {
    #[source] 
    source: io::Error, 
    error: IoError 
}

impl Operand {
    fn read_immediate(input: &mut impl Read, size: Size) -> io::Result<Unsigned> {
        let value = match size {
            Size::X8 => {
                let mut buffer = [0u8; size_of::<u8>() as usize];
                input.read_exact(&mut buffer)?;
                u8::from_le_bytes(buffer) as u64
            },
            Size::X16 => {
                let mut buffer = [0u8; size_of::<u16>() as usize];
                input.read_exact(&mut buffer)?;
                u16::from_le_bytes(buffer) as u64
            },
            Size::X32 => {
                let mut buffer = [0u8; size_of::<u32>() as usize];
                input.read_exact(&mut buffer)?;
                u32::from_le_bytes(buffer) as u64
            },
            Size::X64 => {
                let mut buffer = [0u8; size_of::<u64>() as usize];
                input.read_exact(&mut buffer)?;
                u64::from_le_bytes(buffer) as u64
            }
        };
        
        Ok(Unsigned { value, size })
    }
    
    pub(crate) fn decode(input: &mut impl Read) -> Result<Self, Error> {
        // Try to decode the addressing byte.
        let mut buffer = [0u8; 1];
        input
            .read_exact(&mut buffer)
            .map_err(|source| Error { source, error: IoError::AddressingByte })?;
        
        let addressing_mode = buffer[0] >> 6;
        let size = Size::from_power((buffer[0] & 0b00_11_0000) >> 4);
        // This segment should be used to either encode the offset immediate length or to encode a register.
        let end_segment = buffer[0] & 0b00_00_1111;
        
        // Decode the addressing mode.
        let mode = match addressing_mode {
            AddressingMode::REGISTER_CODE => AddressingMode::Register { register: end_segment },

            AddressingMode::IMMEDIATE_CODE
            | AddressingMode::RELATIVE_CODE => {
                let immediate_size = Size::from_power(end_segment >> 2);
                let immediate = Self::read_immediate(input, immediate_size).map_err(|source| {
                    let error = match addressing_mode {
                        AddressingMode::IMMEDIATE_CODE => IoError::ImmediateValue,
                        AddressingMode::RELATIVE_CODE => IoError::ImmediateOffset,
                        _ => unreachable!()
                    };
                    
                    Error { source, error }
                })?;
                
                match addressing_mode {
                    AddressingMode::IMMEDIATE_CODE => AddressingMode::Immediate { mode: ImmediateAddressing::Immediate { immediate                       }},
                    AddressingMode::RELATIVE_CODE =>  AddressingMode::Immediate { mode: ImmediateAddressing::Relative  { offset: Signed::from(immediate) }},
                    // Addressing mode wasn't modified. The arms here are the same as the parent statement.
                    _ => unreachable!()
                }
            },

            AddressingMode::COMPLEX_CODE => { 
                let complex_mode = Self::decode_complex(input)?;
                AddressingMode::Complex { mode: complex_mode, base: end_segment } 
            },
            // There are 4 possible addressing modes in the first byte. This match covers all of them and the code is 2 
            // bits which guarantees this is unreachable.
            _ => unreachable!()
        };
        
        Ok(Self { size, mode })
    }
    
    fn decode_complex(input: &mut impl Read) -> Result<ComplexAddressing, Error> {
        let mut buffer = [0u8; 1];
        input
            .read_exact(&mut buffer)
            .map_err(|source| Error { source, error: IoError::ComplexAddressingByte })?;
        
        let addressing_mode = (buffer[0] & 0b11_0000_00) >> 6;
        let index_register = (buffer[0] & 0b00_1111_00) >> 2;
        let size = Size::from_power(buffer[0] & 0b00_0000_11);
        
        Ok(match addressing_mode {
            ComplexAddressing::BASE_CODE
            | ComplexAddressing::ARRAY_CODE => match addressing_mode {
                ComplexAddressing::BASE_CODE => ComplexAddressing::Base { mode: BaseAddressing::Base },
                ComplexAddressing::ARRAY_CODE => ComplexAddressing::ArrayAddressing { mode: ArrayAddressing::Array, index: index_register },
                _ => unreachable!()
            },
            ComplexAddressing::BASE_PLUS_OFFSET_CODE
            | ComplexAddressing::OFFSETTED_ARRAY_CODE => {
                let offset = Self::read_immediate(input, size).map_err(|source| Error { source, error: IoError::ImmediateOffset })?;
                match addressing_mode {
                    ComplexAddressing::BASE_PLUS_OFFSET_CODE => ComplexAddressing::Base { mode: BaseAddressing::Offsetted { offset }},
                    ComplexAddressing::OFFSETTED_ARRAY_CODE => ComplexAddressing::ArrayAddressing { mode: ArrayAddressing::Offsetted { offset }, index: index_register },
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        })
    }
    
    fn encode_addressing_byte(self, output: &mut impl Write, addressing_mode: u8, end_segment: u8) -> Result<(), Error> {
        let mut first_byte = addressing_mode << 6;
        first_byte |= (self.size.to_power() & 0b00000011) << 4;
        first_byte |= end_segment & 0b00001111;

        output.write_all(&[first_byte]).map_err(|source| Error { source, error: IoError::AddressingByte })
    }
    
    fn write_immediate(output: &mut impl Write, immediate: Unsigned, error: IoError) -> Result<(), Error> {
        let result = match immediate.size {
            Size::X8 => output.write_all(&(immediate.value as u8).to_le_bytes()),
            Size::X16 => output.write_all(&(immediate.value as u16).to_le_bytes()),
            Size::X32 => output.write_all(&(immediate.value as u32).to_le_bytes()),
            Size::X64 => output.write_all(&(immediate.value as u64).to_le_bytes())
        };
        
        result.map_err(|source| Error { source, error })?;
        Ok(())
    }
    
    pub(crate) fn encode(self, output: &mut impl Write) -> Result<(), Error> {
        match self.mode {
            AddressingMode::Register { register } => self.encode_addressing_byte(output, AddressingMode::REGISTER.code, register)?,
            AddressingMode::Immediate { mode } => {
                self.encode_addressing_byte(output, AddressingMode::REGISTER.code, 0)?;
                
                match mode {
                    ImmediateAddressing::Immediate { immediate } => Self::write_immediate(output, immediate, IoError::ImmediateValue)?,
                    ImmediateAddressing::Relative { offset } => {
                        let immediate = Unsigned::from(offset);
                        Self::write_immediate(output, immediate, IoError::ImmediateOffset)?
                    }
                }
            },
            AddressingMode::Complex { mode, base } => {
                self.encode_addressing_byte(output, AddressingMode::COMPLEX.code, base)?;
                    
                match mode {
                    ComplexAddressing::Base { mode } => match mode {
                        BaseAddressing::Base => {}
                        BaseAddressing::Offsetted { offset } => {}
                    }
                    ComplexAddressing::ArrayAddressing { mode, index } => {}
                }
            }
        }
        
        Ok(())
    }
}