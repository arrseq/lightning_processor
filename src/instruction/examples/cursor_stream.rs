use em_instruction::operation::arithmetic::{ADD_CODE, Arithmetic, SUBTRACT_CODE};
use em_instruction::operation::{ARITHMETIC_CODE, Extension, ExtensionCode, Operation, OperationCode};

fn main() {
	let ext = Extension::try_from(ExtensionCode {
		extension: ARITHMETIC_CODE,
		operation: OperationCode(SUBTRACT_CODE)
	});
	
	dbg!(ext).expect("Error");
}