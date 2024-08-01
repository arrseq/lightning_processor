pub mod encoding;

use crate::instruction::operand::Operand;
use crate::math::vector::Vector4Layout;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperandCategory {
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
    None,
    Lock,
    VectorOperands,
    MapVector { mappings: Vector4Layout, operand: u8 },
    OverrideBranch,
    Destination             { operation: Destination,             destination: Operand },
    Input                   { operation: Input,                   input:       Operand },
    DestinationAndInput     { operation: DestinationAndInput,     destination: Operand, input: Operand },
    DualInput               { operation: DualInput,               inputs: [Operand; 2] },
    DestinationAndDualInput { operation: DestinationAndDualInput, inputs: [Operand; 2],  destination: Operand }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dependencies {
    pub code: u16,
    pub category: Option<OperandCategory>,
    pub executable: bool
}

impl Operation {
    pub const NONE             : Dependencies = Dependencies { code: 0 , category: None                                          , executable: true  };
    pub const LOCK             : Dependencies = Dependencies { code: 1 , category: None                                          , executable: false };
    pub const VECTOR_OPERANDS  : Dependencies = Dependencies { code: 2 , category: None                                          , executable: false };
    pub const MAP_VECTOR       : Dependencies = Dependencies { code: 3 , category: None                                          , executable: false };
    pub const OVERRIDE_BRANCH  : Dependencies = Dependencies { code: 4 , category: None                                          , executable: false };
    pub const STACK            : Dependencies = Dependencies { code: 5 , category: Some(OperandCategory::Input                  ), executable: true  };
    pub const UNSTACK          : Dependencies = Dependencies { code: 6 , category: Some(OperandCategory::Destination            ), executable: true  };
    pub const COPY             : Dependencies = Dependencies { code: 7 , category: Some(OperandCategory::DestinationAndInput    ), executable: true  };
    pub const COMPARE          : Dependencies = Dependencies { code: 8 , category: Some(OperandCategory::DualInput              ), executable: true  };
    pub const SIGNED_COMPARE   : Dependencies = Dependencies { code: 9 , category: Some(OperandCategory::DualInput              ), executable: true  };
    pub const ADD              : Dependencies = Dependencies { code: 10, category: Some(OperandCategory::DestinationAndDualInput), executable: true  };
    pub const FLOATING_ADD     : Dependencies = Dependencies { code: 11, category: Some(OperandCategory::DestinationAndDualInput), executable: true  };
    pub const SUBTRACT         : Dependencies = Dependencies { code: 12, category: Some(OperandCategory::DestinationAndDualInput), executable: true  };
    pub const FLOATING_SUBTRACT: Dependencies = Dependencies { code: 13, category: Some(OperandCategory::DestinationAndDualInput), executable: true  };
    pub const MULTIPLY         : Dependencies = Dependencies { code: 14, category: Some(OperandCategory::DestinationAndDualInput), executable: true  };
    pub const FLOATING_MULTIPLY: Dependencies = Dependencies { code: 15, category: Some(OperandCategory::DestinationAndDualInput), executable: true  };
    pub const DIVIDE           : Dependencies = Dependencies { code: 16, category: Some(OperandCategory::DestinationAndDualInput), executable: true  };
    pub const FLOATING_DIVIDE  : Dependencies = Dependencies { code: 17, category: Some(OperandCategory::DestinationAndDualInput), executable: true  };

    pub const NONE_CODE             : u16 = Self::NONE.code             ;
    pub const LOCK_CODE             : u16 = Self::LOCK.code             ;
    pub const VECTOR_OPERANDS_CODE  : u16 = Self::VECTOR_OPERANDS.code  ;
    pub const MAP_VECTOR_CODE       : u16 = Self::MAP_VECTOR.code       ;
    pub const OVERRIDE_BRANCH_CODE  : u16 = Self::OVERRIDE_BRANCH.code  ;
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
    
    pub const OPERATIONS: [Dependencies; 18] = [
        Self::NONE,
        Self::LOCK,
        Self::VECTOR_OPERANDS, Self::MAP_VECTOR,
        Self::OVERRIDE_BRANCH,
        Self::STACK   , Self::UNSTACK          ,
        Self::COPY    ,
        Self::COMPARE , Self::SIGNED_COMPARE   ,
        Self::ADD     , Self::FLOATING_ADD     , Self::SUBTRACT, Self::FLOATING_SUBTRACT,
        Self::MULTIPLY, Self::FLOATING_MULTIPLY, Self::DIVIDE  , Self::FLOATING_DIVIDE
    ];
}

impl Operation {
    pub fn executable(self) -> bool {
        Self::OPERATIONS[self.code() as usize].executable
    }
}