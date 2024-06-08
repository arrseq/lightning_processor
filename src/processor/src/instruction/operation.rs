use crate::{ExecutionContext, instruction};
use crate::instruction::operation::arithmetic::Arithmetic;

pub mod arithmetic;

pub trait Coded<Type> {
	fn code(&mut self) -> Type;
}

// Extension identifier codes

pub const ARITHMETIC_CODE: u8 = 0;
pub const DATA_CODE      : u8 = 1;

// Operation

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationExecuteError {
	/// Not all operands were provided.
	ExpectedAll,
	/// Static operand was not provided.
	ExpectedStatic,
	/// Dynamic operand was not provided.
	ExpectedDynamic,
	/// No operands should have been provided.
	ExpectedNone
}

pub trait Operation: Coded<u8> {
	/// Whether the operation requires the static operand.
	fn expects_static(&mut self) -> bool;
	/// Whether the operation requires the dynamic operand.
	fn expects_dynamic(&mut self) -> bool;
	
	fn execute(&mut self, code: u8, data: Option<&instruction::Data>, context: &mut ExecutionContext) -> Result<(),
		OperationExecuteError>;

	/// Whether an operand is expected.
	fn expects_operand(&mut self) -> bool {
		self.expects_static() || self.expects_dynamic()
	}
	
	fn expects_all(&mut self) -> bool {
		self.expects_static() && self.expects_dynamic()
	}
	
	fn expects_nothing(&mut self) -> bool {
		!self.expects_dynamic() && !self.expects_static()
	}
	
	fn expects_only_static(&mut self) -> bool {
		self.expects_static() && !self.expects_dynamic()
	}
	
	fn expects_only_dynamic(&mut self) -> bool {
		self.expects_dynamic() && !self.expects_nothing()
	}
}

// Extension
// Used to group operations into categories. Also allows the processor set to be expanded without breaking
// pre-existing code.

pub type ExtensionCode = u8;
pub type OperationCode = u8;

/// Used to indicate that one of the codes were invalid for the target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtensionFromCodeInvalid {
	Extension,
	Operation
}

/// Contains groups of operations which are categorized by extension. This allows for operations to have duplicate
/// names and also allows for the operation set to extended in the future without breaking code that is already
/// compiled for the architecture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Extension {
	Arithmetic(Arithmetic),
}

impl Extension {
	/// Create an extension containing and operation with the extension and operation codes.
	pub fn from_codes(extension: ExtensionCode, operation: OperationCode) -> Result<Self, ExtensionFromCodeInvalid> {
		let invalid_operation = Err(ExtensionFromCodeInvalid::Operation);

		Ok(match extension {
			ARITHMETIC_CODE => Self::Arithmetic(match Arithmetic::from_code(operation) {
				Some(operation) => operation,
				None => return invalid_operation
			}),
			_ => return Err(ExtensionFromCodeInvalid::Extension)
		})
	}
	
	/// Retrieve the underlying operation trait.
	pub fn operation(&mut self) -> &mut impl Operation {
		match self {
			Self::Arithmetic(arithmetic) => arithmetic
		}
	}
}

impl Coded<u8> for Extension {
	fn code(&mut self) -> u8 {
		match self {
			Self::Arithmetic(_) => ARITHMETIC_CODE
		}
	}
}

#[cfg(test)]
mod extension_test {
	use crate::instruction::operation::{ARITHMETIC_CODE, Coded, Extension, Operation};
	use crate::instruction::operation::arithmetic::{ADD_CODE, Arithmetic, SUBTRACT_CODE};

	#[test]
	fn from_codes() {
		let subtract = Extension::from_codes(ARITHMETIC_CODE, SUBTRACT_CODE).unwrap();

		assert_eq!(subtract, Extension::Arithmetic(Arithmetic::Subtract));
		assert_eq!(SUBTRACT_CODE, Arithmetic::Subtract.code());
	}

	#[test]
	fn operation() {
		let mut extension = Extension::from_codes(ARITHMETIC_CODE, ADD_CODE).unwrap();
		let operation_generic = extension.operation();

		assert_eq!(operation_generic.expects_static(), Arithmetic::Add.expects_static());
	}
}