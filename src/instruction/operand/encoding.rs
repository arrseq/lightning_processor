#[cfg(test)]
mod test;

use std::io;
use std::io::Read;
use thiserror::Error;
use crate::instruction::operand::{AddressingMode, ArrayAddressing, BaseAddressing, ComplexAddressing, ImmediateAddressing, Operand};
use crate::math::dynamic_number::{Signed, Size, Unsigned};

#[derive(Debug, Error)]
pub(crate) enum DecodeIoError {
    #[error("Could not retrieve addressing byte")]
    AddressingByte,
    #[error("Could not retrieve immediate value")]
    ImmediateValue,
    #[error("Could not retrieve immediate relative offset")]
    ImmediateOffset,
    #[error("Could not retrieve complex addressing byte")]
    ComplexAddressingByte
}

#[derive(Debug, Error)]
pub(crate) enum DecodeError {
    #[error("Failed to read data for decoding")]
    Io { #[source] source: io::Error, error: DecodeIoError }
}

impl Operand {
    pub(crate) fn decode(input: &mut impl Read) -> Result<Self, DecodeError> {
        // Try to decode the addressing byte.
        let mut buffer = [0u8; 1];
        input
            .read_exact(&mut buffer)
            .map_err(|source| DecodeError::Io { source, error: DecodeIoError::AddressingByte })?;
        
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
                let immediate = Unsigned::read(input, immediate_size).map_err(|source| {
                    let error = match addressing_mode {
                        AddressingMode::IMMEDIATE_CODE => DecodeIoError::ImmediateValue,
                        AddressingMode::RELATIVE_CODE => DecodeIoError::ImmediateOffset,
                        _ => unreachable!()
                    };
                    
                    DecodeError::Io { source, error }
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
    
    fn decode_complex(input: &mut impl Read) -> Result<ComplexAddressing, DecodeError> {
        let mut buffer = [0u8; 1];
        input
            .read_exact(&mut buffer)
            .map_err(|source| DecodeError::Io { source, error: DecodeIoError::ComplexAddressingByte })?;
        
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
                let offset = Unsigned::read(input, size).map_err(|source| DecodeError::Io { source, error: DecodeIoError::ImmediateOffset })?;
                match addressing_mode {
                    ComplexAddressing::BASE_PLUS_OFFSET_CODE => ComplexAddressing::Base { mode: BaseAddressing::Offsetted { offset }},
                    ComplexAddressing::OFFSETTED_ARRAY_CODE => ComplexAddressing::ArrayAddressing { mode: ArrayAddressing::Offsetted { offset }, index: index_register },
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        })
    }
}