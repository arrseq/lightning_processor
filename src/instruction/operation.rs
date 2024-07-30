pub mod encoding;

use crate::instruction::operand::Operand;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Destination {
    Stack
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Input {
    Unstack
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputAndDestination {
    Copy
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DualInput {
    Compare,
    SignedCompare
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DualInputAndDestination {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Destination             { operation: Destination,             destination: Operand },
    Input                   { operation: Input,                   input:       Operand },
    InputAndDestination     { operation: InputAndDestination,     destination: Operand, input: Operand },
    DualInput               { operation: DualInput,               destination: Operand, input: Operand },
    DualInputAndDestination { operation: DualInputAndDestination, input: [Operand; 2],  destination: Operand }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dependencies {
    pub code: u16,
    pub has_destination: bool,
    pub input_count: u8
}

impl Operation {
    pub const STACK_CODE         : u16 = 0;
    pub const UNSTACK_CODE       : u16 = 1;
    pub const COPY_CODE          : u16 = 2;
    pub const COMPARE_CODE       : u16 = 3;
    pub const SIGNED_COMPARE_CODE: u16 = 4;
    pub const ADD_CODE           : u16 = 5;
    pub const SUBTRACT_CODE      : u16 = 6;
    pub const MULTIPLY_CODE      : u16 = 7;
    pub const DIVIDE_CODE        : u16 = 8;

    pub const STACK         : Dependencies = Dependencies { code: Self::STACK_CODE         , has_destination: true,  input_count: 0 };
    pub const UNSTACK       : Dependencies = Dependencies { code: Self::UNSTACK_CODE       , has_destination: false, input_count: 1 };
    pub const COPY          : Dependencies = Dependencies { code: Self::COPY_CODE          , has_destination: true,  input_count: 1 };
    pub const COMPARE       : Dependencies = Dependencies { code: Self::COMPARE_CODE       , has_destination: false, input_count: 2 };
    pub const SIGNED_COMPARE: Dependencies = Dependencies { code: Self::SIGNED_COMPARE_CODE, has_destination: false, input_count: 2 };
    pub const ADD           : Dependencies = Dependencies { code: Self::ADD_CODE           , has_destination: true,  input_count: 2 };
    pub const SUBTRACT      : Dependencies = Dependencies { code: Self::SUBTRACT_CODE      , has_destination: true,  input_count: 2 };
    pub const MULTIPLY      : Dependencies = Dependencies { code: Self::MULTIPLY_CODE      , has_destination: true,  input_count: 2 };
    pub const DIVIDE        : Dependencies = Dependencies { code: Self::DIVIDE_CODE        , has_destination: true,  input_count: 2 };
}