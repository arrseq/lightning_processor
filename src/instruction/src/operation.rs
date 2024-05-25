use crate::operation::arithmetic::Arithmetic;

pub mod arithmetic;

// Extension identifier codes

pub const ARITHMETIC_CODE: u8 = 0;
pub const DATA_CODE: u8 = 1;

// Operation

pub struct OperationCode(pub u8);

pub trait Operation: TryFrom<OperationCode> {
	fn get_code(&mut self) -> u8;
	
	/// Whether the operation requires the static operand.
	fn accepts_static(&mut self) -> bool;
	/// Whether the operation requires the dynamic operand.
	fn accepts_dynamic(&mut self) -> bool;
}

// Extension
// Used to group operations into categories. Also allows the instruction set to be expanded without breaking 
// pre-existing code.

/// Path to a specific operation based on codes.
pub struct ExtensionCode {
	/// Extension code
	pub extension: u8,
	/// Operation code
	pub operation: OperationCode
}

/// Used to indicate that one of the codes were invalid for the target.
#[derive(Debug)]
pub enum ExtensionFromCodeInvalid {
	Extension,
	Operation
}

/// Contains groups of operations which are categorized by extension. This allows for operations to have duplicate 
/// names and also allows for the operation set to extended in the future without breaking code that is already 
/// compiled for the architecture.
#[derive(Debug)]
pub enum Extension {
	Arithmetic(Arithmetic),
	/// Allows movement of data, fetching and storing.
	Data(/* TODO */)
}

impl TryFrom<ExtensionCode> for Extension {
	type Error = ExtensionFromCodeInvalid;
	
	/// Create a new extension with an operation through a path.
	fn try_from(value: ExtensionCode) -> Result<Self, Self::Error> {
		let invalid_operation = Err(ExtensionFromCodeInvalid::Operation);
		
		Ok(match value.extension {
			ARITHMETIC_CODE => Self::Arithmetic(match Arithmetic::try_from(value.operation) {
				Ok(operation) => operation,
				Err(error) => return invalid_operation
			}),
			_ => return Err(ExtensionFromCodeInvalid::Extension)
		})
	}
}