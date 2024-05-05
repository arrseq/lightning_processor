#[derive(Debug)]
pub enum Type {
	Byte,
	Word,
	Dual,
	Quad
}

impl From<Data> for Type {
	fn from(value: Size) -> Self {
		match value {
			Size::Byte(_) => Self::Byte,
			Size::Word(_) => Self::Word,
			Size::Dual(_) => Self::Dual,
			Size::Quad(_) => Self::Quad
		}
	}
}

#[derive(Debug)]
pub enum Data {
	Byte(u8),
	Word(u16),
	Dual(u32),
	Quad(u64)
}

impl From<Type> for Data {
	fn from(value: Type) -> Self {
		match value {
			Type::Byte => Size::Byte(0),
			Type::Word => Size::Word(0),
			Type::Dual => Size::Dual(0),
			Type::Quad => Size::Quad(0)
		}
	}
}