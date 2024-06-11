#[cfg(test)]
mod driver_encoding_test {
	use crate::instruction::{Driver, Driver0Encoding, Driver1Encoding};

	#[test]
	fn extract_extension() { // MOVED
assert_eq!(0b001101_0_0_u8.extract_extension(), 0b00_001101);
assert_eq!(0b101010_0_1_u8.extract_extension(), 0b00_101010);
	}

	#[test]
	fn set_extension() { // MOVED
	/// assert_eq!(0b000000_0_1_u8.set_extension(10), 0b001010_0_1);
	/// assert_eq!(0b101100_0_0_u8.set_extension(0b101100), 0b101100_0_0);
	/// assert_eq!(0b101100_1_0_u8.set_extension(0b101100), 0b101100_1_0);
	/// 
	/// // Truncating extension
	/// assert_eq!(0b00000000_0_0_u8.set_extension(0b11_111111), 0b111111_0_0);
	/// assert_eq!(0b00000000_0_1_u8.set_extension(0b11_111110), 0b111110_0_1);
	}

	#[test]
	fn extract_synchronise() { // MOVED
	/// assert!(0b000000_1_0_u8.extract_synchronise());
	/// assert!(!0b000000_0_0_u8.extract_synchronise());
	/// assert!(0b001010_1_1_u8.extract_synchronise());
	/// assert!(!0b001010_0_1_u8.extract_synchronise());
	}

	#[test]
	fn set_synchronise() {
	/// assert_eq!(0b000000_0_0_u8.set_synchronise(true), 0b000000_1_0);
	/// assert_eq!(0b000000_1_0_u8.set_synchronise(false), 0b000000_0_0);
	/// assert_eq!(0b000000_0_1_u8.set_synchronise(true), 0b000000_1_1);
	/// assert_eq!(0b111111_0_0_u8.set_synchronise(false), 0b111111_0_0);
	}

	#[test]
	fn extract_dynamic_destination() {
	/// assert!(0b000000_0_1_u8.extract_dynamic_destination());
	/// assert!(!0b000000_0_0_u8.extract_dynamic_destination());
	/// assert!(0b000000_1_1_u8.extract_dynamic_destination());
	/// assert!(!0b000000_1_0_u8.extract_dynamic_destination());
	}

	#[test]
	fn set_dynamic_destination() {
	/// assert_eq!(0b000000_0_0_u8.set_dynamic_destination(true), 0b000000_0_1);
	/// assert_eq!(0b000000_1_0_u8.set_dynamic_destination(true), 0b000000_1_1);
	/// assert_eq!(0b000000_0_1_u8.set_dynamic_destination(false), 0b000000_0_0);
	/// assert_eq!(0b000000_1_1_u8.set_dynamic_destination(false), 0b000000_1_0);
	}

	#[test]
	fn extract_operation() {
	/// assert_eq!(0b1101_00_00_u8.extract_operation(), 0b0000_1101);
	/// assert_eq!(0b1010_01_10_u8.extract_operation(), 0b0000_1010);
	}

	#[test]
	fn set_operation() {
	/// assert_eq!(0b0001_00_11_u8.set_operation(0b0000_1111), 0b1111_00_11);
	/// assert_eq!(0b1111_00_10_u8.set_operation(0b0000_1001), 0b1001_00_10);
	/// assert_eq!(0b1010_00_10_u8.set_operation(0b0000_1010), 0b1010_00_10);
	/// 
	/// // Truncating extension
	/// assert_eq!(0b0000_00_00_u8.set_operation(0b1111_1111), 0b1111_00_00);
	/// assert_eq!(0b0000_10_01_u8.set_operation(0b1111_1111), 0b1111_10_01);
	}

	#[test]
	fn extract_addressing() {
	/// assert_eq!(0b0011_10_00_u8.extract_addressing(), 0b000000_10);
	/// assert_eq!(0b1011_11_00_u8.extract_addressing(), 0b000000_11);
	/// assert_eq!(0b0000_00_00_u8.extract_addressing(), 0b000000_00);
	}

	#[test]
	fn set_addressing() {
	/// assert_eq!(0b0000_11_00_u8.set_addressing(0b000000_00), 0b0000_00_00);
	/// assert_eq!(0b0011_00_00_u8.set_addressing(0b000000_01), 0b0011_01_00);
	/// assert_eq!(0b1011_00_00_u8.set_addressing(0b000000_00), 0b1011_00_00);
	/// 
	/// // Truncating extension
	/// assert_eq!(0b0000_00_00_u8.set_addressing(0b111111_11), 0b0000_11_00);
	/// assert_eq!(0b1010_00_01_u8.set_addressing(0b111111_11), 0b1010_11_01);
	}

	#[test]
	fn extract_immediate_exponent() {
	/// assert_eq!(0b0000_00_11_u8.extract_immediate_exponent(), 0b000000_11);
	/// assert_eq!(0b1010_11_01_u8.extract_immediate_exponent(), 0b000000_01);
	}

	#[test]
	fn set_immediate_exponent() {
	/// assert_eq!(0b0011_00_00_u8.set_immediate_exponent(0b000000_11), 0b0011_00_11);
	/// assert_eq!(0b0000_11_00_u8.set_immediate_exponent(0b000000_10), 0b0000_11_10);
	/// assert_eq!(0b1011_01_00_u8.set_immediate_exponent(0b000000_00), 0b1011_01_00);
	/// 
	/// // Truncating extension
	/// assert_eq!(0b0000_00_00_u8.set_immediate_exponent(0b111111_11), 0b0000_00_11);
	/// assert_eq!(0b1011_01_00_u8.set_immediate_exponent(0b111111_10), 0b1011_01_10);
	}
}

#[cfg(test)]
mod driver_test {
	use crate::instruction::Driver;

	#[test]
	fn from_encoded() { // MOVED
	/// let driver = Driver::from_encoded([0b001010_0_1, 0b1111_10_01]);
	/// 
	/// // Driver 0
	/// assert_eq!(driver.extension, 0b001010);
	/// assert!(!driver.synchronise);
	/// assert!(driver.dynamic_destination);
	/// 
	/// // Driver 1
	/// assert_eq!(driver.operation, 0b1111);
	/// assert_eq!(driver.addressing, 0b10);
	/// assert_eq!(driver.immediate_exponent, 0b1);
	}

	#[test]
	fn encode() {
	/// let driver = Driver {
	/// 	operation: 0b1110,
	/// 	extension: 0b1010,
	/// 	synchronise: true,
	/// 	dynamic_destination: false,
	/// 	addressing: 0b11,
	/// 	immediate_exponent: 0b10
	/// };
	/// 
	/// let encoded = driver.encode();
	/// 
	/// assert_eq!(encoded[0], 0b001010_1_0);
	/// assert_eq!(encoded[1], 0b1110_11_10);
	}
}

#[cfg(test)]
mod registers_encoding_test {
	use crate::instruction::{RegistersEncoding};

	#[test]
	fn extract_width() {
	/// assert_eq!(0b11_000_000.extract_width(), 0b000000_11);
	/// assert_eq!(0b10_010_000.extract_width(), 0b000000_10);
	}

	#[test]
	fn set_width() {
		// TODO
	}

	// TODO: More
}

#[cfg(test)]
mod registers_test {
	// TODO: Complete
}

#[cfg(test)]
mod instruction_test {
	use std::io::Cursor;
	use crate::number;
	use crate::instruction::{Data, DecodeError, Destination, Driver, Instruction, Registers};
	use crate::instruction::operand::{AllPresent, CONSTANT_ADDRESSING, Dynamic, IMMEDIATE_EXPONENT_BYTE, Operand,
									  Operands,
									  REGISTER_ADDRESSING};
	use crate::instruction::operation::arithmetic::{ADD_CODE, Arithmetic};
	use crate::instruction::operation::{ARITHMETIC_CODE, Extension};

	#[test]
	fn encode_instruction() {
	/// let driver = Driver {
	/// 	extension: ARITHMETIC_CODE,
	/// 	operation: ADD_CODE,
	/// 	synchronise: true,
	/// 	dynamic_destination: false,
	/// 	addressing: CONSTANT_ADDRESSING,
	/// 	immediate_exponent: IMMEDIATE_EXPONENT_BYTE
	/// };
	/// 
	/// let registers = Registers {
	/// 	width: IMMEDIATE_EXPONENT_BYTE,
	/// 	x_static: 1,
	/// 	x_dynamic: 0
	/// };
	/// 
	/// let target = [ 0b000000_1_0, 0b0000_10_00, 0b00_001_000, 0b00001010 ];
	/// 
	/// assert_eq!(
	/// 	Instruction::encode_driver_registers_immediate(&driver, Some(&registers), Some(&number::Data::Byte(10))
	/// 	).unwrap(),
	/// 	target
	/// );
	}

	#[test]
	fn decode() {
	/// // Decode a valid processor.
	/// let mut driver = Driver {
	/// 	extension: 0,
	/// 	operation: 0,
	/// 	synchronise: true,
	/// 	dynamic_destination: false,
	/// 	addressing: 2,
	/// 	immediate_exponent: 0
	/// };
	/// 
	/// let registers = Registers {
	/// 	width: 0,
	/// 	x_static: 10,
	/// 	x_dynamic: 20
	/// };
	/// 
	/// let mut cursor = Cursor::new(
	/// 	Instruction::encode_driver_registers_immediate(&driver, Some(&registers), Some(&number::Data::Byte(10))
	/// 	).unwrap());
	/// 
	/// let instruction = Instruction::from_encoded(&mut cursor).unwrap();
	/// 
	/// assert!(matches!(instruction.extension, Extension::Arithmetic(_)));
	/// assert!(matches!(instruction.data.unwrap().operands.x_dynamic().unwrap(), Dynamic::Constant
	/// 	(number::Data::Byte(10))));
	/// 
	/// // Synchronous register addressing processor should fail to decode.
	/// driver.addressing = REGISTER_ADDRESSING;
	/// cursor = Cursor::new(
	/// 	Instruction::encode_driver_registers_immediate(&driver, Some(&registers), Some(&number::Data::Byte(10))
	/// 	).unwrap());
	/// let error = Instruction::from_encoded(&mut cursor);
	/// 
	/// assert!(matches!(error, Err(DecodeError::SynchronousRegister)));
	}

	#[test]
	fn destination() {
	/// let x_static = Instruction {
	/// 	extension: Extension::Arithmetic(Arithmetic::Add),
	/// 	data: Some(Data {
	/// 		width: number::Type::Byte,
	/// 		destination: Destination::Static,
	/// 		synchronise: false,
	/// 		operands: Operands::AllPresent(AllPresent {
	/// 			x_static: 0,
	/// 			x_dynamic: Dynamic::Register(1)
	/// 		})
	/// 	})
	/// };
	/// 
	/// let x_dynamic = Instruction {
	/// 	extension: Extension::Arithmetic(Arithmetic::Add),
	/// 	data: Some(Data {
	/// 		width: number::Type::Byte,
	/// 		destination: Destination::Dynamic,
	/// 		synchronise: false,
	/// 		operands: Operands::AllPresent(AllPresent {
	/// 			x_static: 0,
	/// 			x_dynamic: Dynamic::Register(1)
	/// 		})
	/// 	})
	/// };
	/// 
	/// assert!(matches!(x_static.destination().unwrap(), Operand::Static(_)));
	/// assert!(!matches!(x_dynamic.destination().unwrap(), Operand::Static(_)));
	}
}