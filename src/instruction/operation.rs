pub mod encoding;

use crate::instruction::operand::Operand;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Category {
    Destination,
    Input,
    DestinationAndInput,
    DualInput,
    DestinationAndDualInput
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Destination {
    Unstack
}

impl Destination {
    pub const UNSTACK: u16 = Operation::UNSTACK.code;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Input {
    Stack
}

impl Input {
    pub const STACK: u16 = Operation::STACK.code;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DestinationAndInput {
    Copy
}

impl DestinationAndInput {
    pub const COPY: u16 = Operation::COPY.code;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DualInput {
    Compare,
    SignedCompare
}

impl DualInput {
    pub const COMPARE: u16 = Operation::COMPARE.code;
    pub const SIGNED_COMPARE: u16 = Operation::SIGNED_COMPARE.code;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DestinationAndDualInput {
    Add,
    FloatingAdd,
    Subtract,
    FloatingSubtract,
    Multiply,
    FloatingMultiply,
    Divide,
    FloatingDivide
}

impl DestinationAndDualInput {
    pub const ADD: u16 = Operation::ADD.code;
    pub const FLOATING_ADD: u16 = Operation::FLOATING_ADD.code;
    pub const SUBTRACT: u16 = Operation::SUBTRACT.code;
    pub const FLOATING_SUBTRACT: u16 = Operation::FLOATING_SUBTRACT.code;
    pub const MULTIPLY: u16 = Operation::MULTIPLY.code;
    pub const FLOATING_MULTIPLY: u16 = Operation::FLOATING_MULTIPLY.code;
    pub const DIVIDE: u16 = Operation::DIVIDE.code;
    pub const FLOATING_DIVIDE: u16 = Operation::FLOATING_DIVIDE.code;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Destination             { operation: Destination,             destination: Operand },
    Input                   { operation: Input,                   input:       Operand },
    DestinationAndInput     { operation: DestinationAndInput,     destination: Operand, input: Operand },
    DualInput               { operation: DualInput,               input: [Operand; 2] },
    DestinationAndDualInput { operation: DestinationAndDualInput, input: [Operand; 2],  destination: Operand }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dependencies {
    pub code: u16,
    pub category: Category
}

impl Operation {
    pub const STACK            : Dependencies = Dependencies { code: 0 , category: Category::Input                   };
    pub const UNSTACK          : Dependencies = Dependencies { code: 1 , category: Category::Destination             };
    pub const COPY             : Dependencies = Dependencies { code: 2 , category: Category::DestinationAndInput     };
    pub const COMPARE          : Dependencies = Dependencies { code: 3 , category: Category::DualInput               };
    pub const SIGNED_COMPARE   : Dependencies = Dependencies { code: 4 , category: Category::DualInput               };
    pub const ADD              : Dependencies = Dependencies { code: 5 , category: Category::DestinationAndDualInput };
    pub const FLOATING_ADD     : Dependencies = Dependencies { code: 6 , category: Category::DestinationAndDualInput };
    pub const SUBTRACT         : Dependencies = Dependencies { code: 7 , category: Category::DestinationAndDualInput };
    pub const FLOATING_SUBTRACT: Dependencies = Dependencies { code: 8 , category: Category::DestinationAndDualInput };
    pub const MULTIPLY         : Dependencies = Dependencies { code: 9 , category: Category::DestinationAndDualInput };
    pub const FLOATING_MULTIPLY: Dependencies = Dependencies { code: 10, category: Category::DestinationAndDualInput };
    pub const DIVIDE           : Dependencies = Dependencies { code: 11, category: Category::DestinationAndDualInput };
    pub const FLOATING_DIVIDE  : Dependencies = Dependencies { code: 12, category: Category::DestinationAndDualInput };

    pub const STACK_CODE            : u16 = Self::STACK.code            ;
    pub const UNSTACK_CODE          : u16 = Self::UNSTACK.code          ;
    pub const COPY_CODE             : u16 = Self::COPY.code             ;
    pub const COMPARE_CODE          : u16 = Self::COMPARE.code          ;
    pub const SIGNED_COMPARE_CODE   : u16 = Self::SIGNED_COMPARE.code   ;
    pub const ADD_CODE              : u16 = Self::ADD.code              ;
    pub const FLOATING_ADD_CODE     : u16 = Self::FLOATING_ADD.code     ;
    pub const SUBTRACT_CODE         : u16 = Self::SUBTRACT.code         ;
    pub const FLOATING_SUBTRACT_CODE: u16 = Self::FLOATING_SUBTRACT.code;
    pub const MULTIPLY_CODE         : u16 = Self::MULTIPLY.code         ;
    pub const FLOATING_MULTIPLY_CODE: u16 = Self::FLOATING_MULTIPLY.code;
    pub const DIVIDE_CODE           : u16 = Self::DIVIDE.code           ;
    pub const FLOATING_DIVIDE_CODE  : u16 = Self::FLOATING_DIVIDE.code  ;
    
    pub const OPERATIONS: [Dependencies; 13] = [
        Self::STACK   , Self::UNSTACK          ,
        Self::COPY    ,
        Self::COMPARE , Self::SIGNED_COMPARE   ,
        Self::ADD     , Self::FLOATING_ADD     , Self::SUBTRACT, Self::FLOATING_SUBTRACT,
        Self::MULTIPLY, Self::FLOATING_MULTIPLY, Self::DIVIDE  , Self::FLOATING_DIVIDE
    ];
}