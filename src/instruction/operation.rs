//! Operation codes.
//! Module contains all operations for the architecture that are classified into behavioral groups. When referring to
//! moving data from one operand to another, this means the operand data has already been processed
//! and the value has been extracted from where it references.

use super::operand::Destination;

/// Memory based operations.
pub enum Memory {
	/// Move data from the operand not targeted from [Destination] state.
	/// - If [Destination::First] then clone the second operand into the first.
	/// - If [Destination::Second] then clone the first operand into the second.
	Clone	
}

pub enum Numerical {
	Add,
	Subtract,
	Multiply,
	Divide
}

pub enum IntegerSign {
	Negate,
	InvertSign
}

pub enum Bitwise {
	And,
	Or,
	ExclusiveOr
}

/// Classified based on similar function and behavior. 
pub enum Classification {
	Memory(Memory),
	Integer(Numerical),
	IntegerSign(IntegerSign),
	Magnitude(Numerical),
	Bitwise(Bitwise)
}