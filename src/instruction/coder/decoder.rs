//! Instruction Binary Decoder
//! Unit used for decoding instructions from binary streams.

use std::io;
use std::io::Read;
use crate::instruction::{Instruction, operand};
use crate::instruction::operand::Destination;
use crate::instruction::operation::{Classification, Invalid, Logical, Memory, Numerical, RawOperationTarget,  
									StorageMode, IntegerSign};

#[derive(Debug)]
pub enum SyntaxError {
	/// Invalid classification, beyond the range of supported classifiers.
	InvalidClassification,
	/// Invalid operation because the operation does not exist in the specific classification.
	InvalidOperation
}

#[derive(Debug)]
pub enum Error {
	/// The stream failed to perform an operation.
	Stream(io::Error),
	/// The byte stream did not supply enough bytes.
	Supply,
	/// The syntax was incorrect. 
	Syntax(SyntaxError)
}

/// Decode instruction from binary stream into an instruction. The result is written to the instruction's mutable 
/// reference.
pub fn decode(stream: &mut impl Read, instruction: &mut Instruction) -> Result<(), Error> {
	// First 2 bytes for primary instruction control.
	let mut control_bytes = [0u8; 2];
	match stream.read(&mut control_bytes) {
		Err(error) => return Err(Error::Stream(error)),
		Ok(result) => {
			if result != control_bytes.len() {
				return Err(Error::Supply);
			}
		}
	};

	// Operation and flow.

	// 7 most significant bits are the classifier identifier. 
	instruction.operation = {
		let raw = RawOperationTarget {
			classification: control_bytes[0] >> 1,
			operation: control_bytes[1] >> 4
		};
		
		println!("{}, {}", raw.classification, raw.operation);
		
		match Classification::try_from(raw) {
			Err(error) => return Err(match error {
				Invalid::Classification => Error::Syntax(SyntaxError::InvalidClassification),
				Invalid::Operation => Error::Syntax(SyntaxError::InvalidOperation)
			}), 
			Ok(result) => result
		}
	};
	
	// Read last bit and match to a destination enum.
	instruction.operands.destination = match control_bytes[0] & 0b0000000_1 {
		0 => Destination::First,
		_ => Destination::Second, // This counts at doing 1 => ... because the arm cannot match anything other than 
		// 0-1.
	};
	
	// Addressing
	{
		let capture_mask = 0b00000011;
		// The method used for addressing and what it mainly affects.
		// The bits 4 and 5 contain the value.
		let method = control_bytes[1] >> 2 & capture_mask;
		
		// The specific mode the addressing method should be in.
		// Last 2 bits contain the value.
		let mode = control_bytes[1] & capture_mask;
		
		// Determine the parameter signature
		let storage_mode = match instruction.operation {
			Classification::Memory(_) => Memory::get_storage(),
			Classification::Integer(_) | Classification::Magnitude(_) => Numerical::get_storage(),
			Classification::IntegerSign(_) => IntegerSign::get_storage(),
			Classification::Logical(_) => Logical::get_storage()
		};
		
		match storage_mode {
		    operand::StorageMode::Full => todo!(),
			
		}
	}
	
	Ok(())
}