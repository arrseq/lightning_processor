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

/// # Operation table
/// | mnemonic       | u16 code | has destination | number of inputs |
/// | -------------- | -------- | --------------- | ---------------- |
/// | stack          | 0        | yes             | 0                |
/// | unstack        | 1        | no              | 1                |
/// | copy           | 2        | yes             | 1                |
/// | compare        | 3        | no              | 2                |
/// | signed compare | 4        | no              | 2                |
/// | add            | 5        | yes             | 2                |
/// | subtract       | 6        | yes             | 2                |
/// | multiply       | 7        | yes             | 2                |
/// | divide         | 8        | yes             | 2                |
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
    pub const STACK         : Dependencies = Dependencies { code: 0, has_destination: true,  input_count: 0 };
    pub const UNSTACK       : Dependencies = Dependencies { code: 1, has_destination: false, input_count: 1 };
    pub const COPY          : Dependencies = Dependencies { code: 2, has_destination: true,  input_count: 1 };
    pub const COMPARE       : Dependencies = Dependencies { code: 3, has_destination: false, input_count: 2 };
    pub const SIGNED_COMPARE: Dependencies = Dependencies { code: 4, has_destination: false, input_count: 2 };
    pub const ADD           : Dependencies = Dependencies { code: 5, has_destination: true,  input_count: 2 };
    pub const SUBTRACT      : Dependencies = Dependencies { code: 6, has_destination: true,  input_count: 2 };
    pub const MULTIPLY      : Dependencies = Dependencies { code: 7, has_destination: true,  input_count: 2 };
    pub const DIVIDE        : Dependencies = Dependencies { code: 8, has_destination: true,  input_count: 2 };
}