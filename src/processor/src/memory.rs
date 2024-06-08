use crate::number;
use crate::number::{ExtractError, QUAD_SIZE, UnsignedByteFrame};

pub const LINE_ADDRESS_MASK: u64 = u64::MAX << 3;
pub const BYTE_ADDRESS_MASK: u64 = !LINE_ADDRESS_MASK;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtError {
	/// The memory address requested data that is sized outside the memory aligned divisions.
	UnalignedAccess,
	/// Data was accessed in a range outside of memory
	OutOfBounds,
	/// Failed to extract bytes from the memory line.
	Extract(ExtractError)
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Memory {
	/// Store as lines to save memory addresses during execution. To extract individual bytes, we need to use
	/// bitmasks. The addresses are encoded in order in a single line.
	///
	/// All bits except the last 3 are used for addressing the line. The suffix 3 bits are used to select the byte in
	/// the line.
	pub lines: Vec<u64>
}

impl Memory {
	pub fn from_size(size: u64) -> Self {
		Self {
			lines: vec![0; size as usize]
		}
	}
	
	pub fn at(&self, address: u64, size: number::Type) -> Result<number::Data, AtError> {
		// TODO: Virtual memory
		let line_address = (LINE_ADDRESS_MASK & address) as usize;
		let byte_address = (BYTE_ADDRESS_MASK & address) as u8;
		
		let line = match self.lines.get(line_address) {
			Some(line) => line,
			None => return Err(AtError::OutOfBounds)
		};
		
		Ok(match UnsignedByteFrame::extract(*line, byte_address, size) {
			Err(error) => return Err(AtError::Extract(error)),
			Ok(data) => data
		})
	}
}

#[cfg(test)]
mod memory_test {
}