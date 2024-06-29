use emulator::processor;
use crate::emulator::processor::processor::instruction;
use crate::emulator::processor::processor::instruction::operation::arithmetic::Arithmetic;
use crate::utility::Coded;

use super::operand::OperandsPresence;

pub mod arithmetic;

// Extension identifier codes

pub const ARITHMETIC_CODE: u8 = 0;
pub const DATA_CODE      : u8 = 1;

// Operation

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationExecuteError {
    /// The data parameter received the wrong value for the current operations. The boolean in the error contains
    /// whether the data parameter was expected.
    Data(bool),
    /// The operand presence was incorrect. The expected operand presence is contained in this error.
    Operand(OperandsPresence)
}

pub trait Operation<'a>: Coded<u8> + Default {
    fn execute(&mut self, code: u8, data: Option<&instruction::Data>, context: &mut processor::processor::Context) -> Result<(),
        OperationExecuteError>;

    /// Get which operands are expected.
    fn get_presence(&mut self) -> OperandsPresence;
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

impl Default for Extension {
    fn default() -> Self {
        Self::Arithmetic(Arithmetic::default())
    }
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

// TODO: Moved to doctest
#[cfg(test)]
mod extension_test {
    use crate::emulator::processor::processor::instruction::operation::{ARITHMETIC_CODE, Coded, Extension, Operation};
    use crate::emulator::processor::processor::instruction::operation::arithmetic::{ADD_CODE, Arithmetic, SUBTRACT_CODE};

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

        // assert_eq!(operation_generic.expects_static(), Arithmetic::Add.expects_static());
    }
}