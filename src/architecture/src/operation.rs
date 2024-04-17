/// Most function is register based.
pub enum Code {
    Terminate,
    Interrupt,
    Safe,

    // Data
    LoadFromImmediate,
    LoadFromMemory,
    LoadToMemory,
    CloneToRegister,

    // Arithmetic
    Add,
    AddFloat,
    AddDouble,

    Subtract,
    SubtractFloat,
    SubtractDouble,

    Multiply, 
    MultiplyInteger, 
    MultiplyFloat, 
    MultiplyDouble, 

    Divide,
    DivideInteger,
    DivideFloat,
    DivideDouble,
    
    And,
    Or,
    ExclusiveOr,
    Not,
    ShiftLeastSignificant,
    ShiftMostSignificant,
    TrailingZeros,

    // Branching
    Branch,
    BranchEqual,
    BranchUnequal,
    BranchGreater,
    BranchGreaterThanOrEqual,
}