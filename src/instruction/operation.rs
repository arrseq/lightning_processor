use crate::instruction::operand::Operand;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Destination {
    RetrieveFromStack
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Input {
    AppendToStack
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputAndDestination {
    Copy
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DualInput {
    SignedCompared,
    UnsignedCompare
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Destination         { operation: Destination,         destination: Operand },
    Input               { operation: Input,               input:       Operand },
    InputAndDestination { operation: InputAndDestination, destination: Operand, input: Operand },
    DualInput           { operation: DualInput,           destination: Operand, input: Operand }
}