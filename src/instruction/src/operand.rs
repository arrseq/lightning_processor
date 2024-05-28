use crate::{absolute};

// Single

/// A register code. This is static because this only serves as a register code operand and can only be used to 
/// dereference a register. Instruction executors never get access to this value directly, instead they get a 
/// register target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Static(pub Option<u8>);

/// There are no guarantees that this code is valid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawRegisterCode(pub u8);

/// Either a register code or immediate value addressing mode. Being dynamic means this gives the programmer freedom to 
/// pick either of the addressing modes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dynamic {
	None,
	Register(u8),
	Offset(absolute::Data),
	Constant(absolute::Data),
	Address(absolute::Data)
}

/// Operands provide the operation the arguments necessary for computing, There are 2 types of operands, static and 
/// dynamic operands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
	Static(Static),
	Dynamic(Dynamic)
}

// Instruction ready operand parameter that contains addressing for a different modes of having operands.

/// All operands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllPresent {
	pub x_static: Static,
	pub x_dynamic: Dynamic
}

/// Multi configuration of operands for an instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operands {
	None,
	AllPresent(AllPresent),
	Static(Static),
	Dynamic(Dynamic)
}