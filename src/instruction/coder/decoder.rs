//! Instruction Binary Decoder
//! Unit used for decoding instructions from binary streams.

use std::io;
use std::io::Read;
use crate::instruction::Instruction;
use crate::instruction::operand::Destination;
use crate::instruction::operation::{Classification, Invalid, RawOperationTarget};

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

	// 7 most significant bits are the classifier identifier. 
	let classification = {
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
	let destination = match control_bytes[0] & 0b0000000_1 {
		0 => Destination::First,
		_ => Destination::Second, // This counts at doing 1 => ... because the arm cannot match anything other than 
		// 0-1.
	};
	
	println!("{:?}", classification);

	Ok(())
}