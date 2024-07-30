pub mod encoding;

use crate::instruction::operand::Operand;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Category {
    Destination,
    Input,
    InputAndDestination,
    DualInput,
    DualInputAndDestination
}

impl Category {
    pub fn has_destination(self) -> bool {
        matches!(self, Category::Destination | Category::InputAndDestination | Category::DualInputAndDestination)
    }
}

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
    pub category: Category
}

impl Operation {
    pub const STACK            : Dependencies = Dependencies { code: 0 , category: Category::Input                   };
    pub const UNSTACK          : Dependencies = Dependencies { code: 1 , category: Category::Destination             };
    pub const COPY             : Dependencies = Dependencies { code: 2 , category: Category::InputAndDestination     };
    pub const COMPARE          : Dependencies = Dependencies { code: 3 , category: Category::DualInput               };
    pub const SIGNED_COMPARE   : Dependencies = Dependencies { code: 4 , category: Category::DualInput               };
    pub const ADD              : Dependencies = Dependencies { code: 5 , category: Category::DualInputAndDestination };
    pub const FLOATING_ADD     : Dependencies = Dependencies { code: 6 , category: Category::DualInputAndDestination };
    pub const SUBTRACT         : Dependencies = Dependencies { code: 7 , category: Category::DualInputAndDestination };
    pub const FLOATING_SUBTRACT: Dependencies = Dependencies { code: 8 , category: Category::DualInputAndDestination };
    pub const MULTIPLY         : Dependencies = Dependencies { code: 9 , category: Category::DualInputAndDestination };
    pub const FLOATING_MULTIPLY: Dependencies = Dependencies { code: 10, category: Category::DualInputAndDestination };
    pub const DIVIDE           : Dependencies = Dependencies { code: 11, category: Category::DualInputAndDestination };
    pub const FLOATING_DIVIDE  : Dependencies = Dependencies { code: 12, category: Category::DualInputAndDestination };

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