use em_instruction::operation::arithmetic::{Arithmetic, SUBTRACT_CODE};
use em_instruction::operation::{ARITHMETIC_CODE, Extension, ExtensionCode, Operation, RawOperationCode};

fn main() {
	let subtract = Extension::try_from(ExtensionCode {
		extension: ARITHMETIC_CODE,
		operation: RawOperationCode(SUBTRACT_CODE)
	}).unwrap();

	assert_eq!(subtract, Extension::Arithmetic(Arithmetic::Subtract));
	assert_eq!(SUBTRACT_CODE, Arithmetic::Subtract.get_code())
}