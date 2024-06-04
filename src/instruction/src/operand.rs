use crate::{absolute};

// Single

/// A register code. This is static because this only serves as a register code operand and can only be used to 
/// dereference a register. Instruction executors never get access to this value directly, instead they get a 
/// register target.
pub type Static = u8;

/// Either a register code or immediate value addressing mode. Being dynamic means this gives the programmer freedom to 
/// pick either of the addressing modes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dynamic {
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
	AllPresent(AllPresent),
	Static(Static),
	Dynamic(Dynamic)
}

impl Operands {
	/// Try to get the static operand.
	pub fn x_static(&self) -> Option<Static> {
		Some(match self {
			Self::Static(x_static) => *x_static,
			Self::AllPresent(x_all) => x_all.x_static,
			_ => return None
		})
	}

	/// Try to get the dynamic operand.
	pub fn x_dynamic(&self) -> Option<&Dynamic> {
		Some(match self {
			Self::Dynamic(x_dynamic) => x_dynamic,
			Self::AllPresent(x_all) => &x_all.x_dynamic,
			_ => return None
		})
	}
}

#[cfg(test)]
mod operands_test {
	use crate::absolute;
	use crate::operand::{AllPresent, Dynamic, Operands};

	#[test]
	fn x_static() {
		let x_static = 5;
		
		let all = Operands::AllPresent(AllPresent {
		    x_static,
		    x_dynamic: Dynamic::Constant(absolute::Data::Byte(5))
		});
		
		let static_only = Operands::Static(x_static);
		let dynamic_only = Operands::Dynamic(Dynamic::Constant(absolute::Data::Byte(5)));
		
		assert_eq!(all.x_static().unwrap(), x_static);
		assert_eq!(static_only.x_static().unwrap(), x_static);
		assert!(dynamic_only.x_static().is_none());
	}
	
	#[test]
	fn x_dynamic() {
		let x_dynamic = Dynamic::Constant(absolute::Data::Byte(5));
		
		let all = Operands::AllPresent(AllPresent {
		    x_static: 10,
		    x_dynamic: x_dynamic.clone()
		});
		
		let static_only = Operands::Static(10);
		let dynamic_only = Operands::Dynamic(x_dynamic.clone());
		
		assert_eq!(*all.x_dynamic().unwrap(), x_dynamic);
		assert_eq!(*dynamic_only.x_dynamic().unwrap(), x_dynamic);
		assert!(static_only.x_dynamic().is_none());
	}
}