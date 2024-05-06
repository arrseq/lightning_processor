pub enum Memory {
	Move	
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
	MemoryOperation(Memory),
	Integer(Numerical),
	IntegerSign(IntegerSign),
	Magnitude(Numerical),
	Bitwise(Bitwise)
}