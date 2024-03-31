pub trait Mutable {
    type Type;

    fn get(&mut self) -> Self::Type;
    fn set(&mut self, value: Self::Type);
}

pub trait TransformerMutable {
    type TypeIn;
    type TypeOut;

    fn get(&mut self) -> Self::TypeOut;
    fn set(&mut self, value: Self::TypeIn);
}