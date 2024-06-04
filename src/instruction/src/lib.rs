//! Binary instruction format is as follows.
//! TODO: Explain each further
//! Driver 0:
//! - Extension
//! - Synchronise
//! - Destination Dynamic 
//! 
//! Driver 1
//! - Operation
//! - Addressing
//! - Addressing Parameter
//! 
//! Data:
//! - Width
//! - Static Operand
//! - Dynamic Operand
//! 
//! Immediate 0..8 quantized to 0, 2, 4 and 8.

#![allow(clippy::unusual_byte_groupings)]

pub mod absolute;
pub mod operand;
pub mod operation;

use std::io;
use std::io::Read;
use crate::operand::{Operand, Operands};
use crate::operation::{Extension, ExtensionFromCodeInvalid};

// region: Binary instruction bit masks
pub const DRIVER0_EXTENSION_MASK : u8 = 0b111111_0_0;
pub const DRIVER0_SYNCHRONISE_MASK : u8 = 0b000000_1_0;
pub const DRIVER0_DYNAMIC_DESTINATION: u8 = 0b000000_0_1;

pub const DRIVER1_OPERATION_MASK : u8 = 0b1111_00_00;
pub const DRIVER1_ADDRESSING_MASK : u8 = 0b0000_11_00;
pub const DRIVER1_ADDRESSING_PARAMETER: u8 = 0b0000_00_11;

pub const DATA_WIDTH_MASK : u8 = 0b11_000_000;
pub const DATA_STATIC_OPERAND_MASK: u8 = 0b00_111_000;
pub const DATA_DYNAMIC_OPERAND_MASK: u8 = 0b00_000_111;
// endregion

/// The operand to dereference store the operation result in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Destination {
    Static,
    Dynamic
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub operation: Extension,
    /// Width of operands when dereferenced and for storing result.
    pub width: absolute::Type,
    pub destination: Destination,
    pub operands: Operands,
}

#[derive(Debug)]
pub enum DecodeError {
    /// Caused by the stream.
    Stream(io::Error),
    /// The extension of operation code was invalid.
    InvalidCode(ExtensionFromCodeInvalid)
}

/// Caused by using a destination which corresponds to an operand that is not provided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DestinationError {
    /// The static operand wasn't present.
    Static,
    /// The dynamic operand wasn't present.
    Dynamic
}

/// Structured data from the driver bytes.
pub struct Driver {
    /// Operation extension
    pub extension: u8,
    pub operation: u8,
    /// Whether the instruction should be synchronously executed in respect to other parallel processors. 
    pub synchronise: bool,
    /// Whether to store the data where the dynamic operand points if its addressing mode supports it.
    pub dynamic_destination: bool,
    /// Addressing mode of the dynamic operand
    pub addressing: u8,
    /// Parameter for the addressing mode.
    pub addressing_parameter: u8
}

impl Driver {
    pub fn extract_extension(driver0: u8) -> u8 {
        (DRIVER0_EXTENSION_MASK & driver0) >> 2
    }

    pub fn set_extension(driver0: u8, extension: u8) -> u8 {
        let layer = (0b00_111111 & extension) << 2;
        (!DRIVER0_EXTENSION_MASK & driver0) | layer
    }
    
    pub fn extract_synchronise(driver0: u8) -> bool {
        // Value will always be 1 bit.
        let bit = (DRIVER0_SYNCHRONISE_MASK & driver0) >> 1;
        bit == 1
    }

    pub fn set_synchronise(driver0: u8, lock: bool) -> u8 {
        let layer = (lock as u8) << 1;
        (!DRIVER0_SYNCHRONISE_MASK & driver0) | layer
    }
    
    pub fn extract_dynamic_destination(driver0: u8) -> bool {
        // Value will always be 1 bit.
        (DRIVER0_DYNAMIC_DESTINATION & driver0) == 1
    }
    
    pub fn set_dynamic_destination(driver0: u8, dynamic_destination: bool) -> u8 {
        (!DRIVER0_DYNAMIC_DESTINATION & driver0) | dynamic_destination as u8
    }
    
    pub fn from_bytes(bytes: [u8; 2]) -> Self {
        let driver0 = bytes[0];
        let driver1 = bytes[1];

        Driver {
            extension: Driver::extract_extension(driver0),
            operation: 0,
            synchronise: Driver::extract_synchronise(driver0),
            dynamic_destination: Driver::extract_dynamic_destination(driver0),
            addressing: 0,
            addressing_parameter: 0,
        }
    }
    
    pub fn encode_bytes(self) -> [u8; 2] {
        let mut driver0 = Driver::set_extension(0, self.extension);
        driver0 = Driver::set_synchronise(driver0, self.synchronise);
        driver0 = Driver::set_dynamic_destination(driver0, self.dynamic_destination);
        
        // TODO
        [driver0, 0]
    }
}

impl Instruction {
    // Decode a binary stream to update this instruction with the new data.
    pub fn decode(&mut self, stream: &mut dyn Read) -> Result<(), DecodeError> {
        let buffer = [0u8; 1];

        // Decode driver 0


        Ok(())
    }

    /// Get the operand that the destination property corresponds to.
    /// ```
    /// use em_instruction::{Destination, Instruction};
    /// use em_instruction::absolute::{Data, Type};
    /// use em_instruction::operand::{AllPresent, Dynamic, Operand, Operands};
    /// use em_instruction::operation::arithmetic::Arithmetic;
    /// use em_instruction::operation::Extension;
    ///
    /// let mut instruction = Instruction {
    ///     operation: Extension::Arithmetic(Arithmetic::Add),
    ///     width: Type::Byte,
    ///     destination: Destination::Static,
    ///     operands: Operands::AllPresent(AllPresent {
    ///         x_static: 10,
    ///         x_dynamic: Dynamic::Constant(Data::Byte(5))
    ///     })
    /// };
    ///
    /// assert!(match instruction.destination().unwrap() {
    ///     Operand::Static(_) => true,
    ///     _ => false
    /// });
    ///
    /// instruction.destination = Destination::Dynamic;
    ///
    /// assert!(match instruction.destination().unwrap() {
    ///     Operand::Static(_) => false,
    ///     _ => true
    /// });
    /// ```
    pub fn destination(&self) -> Result<Operand, DestinationError> {
        Ok(match self.destination {
            Destination::Static => match self.operands.x_static() {
                Some(x_static) => Operand::Static(x_static),
                None => return Err(DestinationError::Static)
            },
            Destination::Dynamic => match self.operands.x_dynamic() {
                Some(x_dynamic) => Operand::Dynamic(x_dynamic.clone()),
                None => return Err(DestinationError::Dynamic)
            }
        })
    }
}

#[cfg(test)]
mod driver_test {
    use crate::Driver;

    #[test]
    fn test_extract_extension() {
        assert_eq!(Driver::extract_extension(0b001101_0_0), 0b00_001101);
        assert_eq!(Driver::extract_extension(0b101010_0_1), 0b00_101010);
    }

    #[test]
    fn test_set_extension() {
        assert_eq!(Driver::set_extension(0b000000_0_1, 0b001010), 0b001010_0_1);
        assert_eq!(Driver::set_extension(0b101100_0_0, 0b101100), 0b101100_0_0);
        assert_eq!(Driver::set_extension(0b101100_1_0, 0b101100), 0b101100_1_0);

        // Truncating extension
        assert_eq!(Driver::set_extension(0b00000000_0_0, 0b11_111111), 0b111111_0_0);
        assert_eq!(Driver::set_extension(0b00000000_0_1, 0b11_111110), 0b111110_0_1);
    }

    #[test]
    fn test_extract_synchronise() {
        assert!(Driver::extract_synchronise(0b000000_1_0));
        assert!(!Driver::extract_synchronise(0b000000_0_0));
        assert!(Driver::extract_synchronise(0b001010_1_1));
        assert!(!Driver::extract_synchronise(0b001010_0_1));
    }

    #[test]
    fn test_set_synchronise() {
        assert_eq!(Driver::set_synchronise(0b000000_0_0, true), 0b000000_1_0);
        assert_eq!(Driver::set_synchronise(0b000000_1_0, false), 0b000000_0_0);
        assert_eq!(Driver::set_synchronise(0b000000_0_1, true), 0b000000_1_1);
        assert_eq!(Driver::set_synchronise(0b111111_0_0, false), 0b111111_0_0);
    }

    #[test]
    fn test_extract_dynamic_destination() {
        assert!(Driver::extract_dynamic_destination(0b000000_0_1));
        assert!(!Driver::extract_dynamic_destination(0b000000_0_0));
        assert!(Driver::extract_dynamic_destination(0b000000_1_1));
        assert!(!Driver::extract_dynamic_destination(0b000000_1_0));
    }
    
    #[test]
    fn test_set_dynamic_destination() {
        assert_eq!(Driver::set_dynamic_destination(0b000000_0_0, true), 0b000000_0_1);
        assert_eq!(Driver::set_dynamic_destination(0b000000_1_0, true), 0b000000_1_1);
        assert_eq!(Driver::set_dynamic_destination(0b000000_0_1, false), 0b000000_0_0);
        assert_eq!(Driver::set_dynamic_destination(0b000000_1_1, false), 0b000000_1_0);
    }
    
    #[test]
    fn test_from_bytes() {
        let driver0 = 0b001010_0_0;
        let driver1 = 0b1111_00_00;
        let driver = Driver::from_bytes([driver0, 0]);
        
        assert_eq!(driver.extension, 0b001010);
    }
}

#[cfg(test)]
mod instruction_test {}