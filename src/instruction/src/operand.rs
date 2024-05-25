use crate::absolute;

/// A register code. This is static because this only serves as a register code operand and can only be used to 
/// dereference a register. Instruction executors never get access to this value directly, instead they get a 
/// register target.
pub struct Static(pub u8);

/// Either a register code or immediate value addressing mode. Being dynamic means this gives the programmer freedom to 
/// pick either of the addressing modes.
pub enum Dynamic {
	/// Register code.
	Register(u8),
	/// Immediate value.
	Immediate(absolute::Data)
}

/// Operands provide the operation the arguments necessary for computing, There are 2 types of operands, static and 
/// dynamic operands.
pub enum Operand {
	Static(Static),
	Dynamic(Dynamic)
}