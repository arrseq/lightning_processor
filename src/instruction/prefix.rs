use instruction::operation;

pub enum Repeat {
    /// Repeat this instruction a fixed number of times based on the value of the A register.
    Fixed,
    /// Repeat this instruction until the A register is equal to the B register.
    UntilEqual
}

pub enum Prefix {
    /// Escape into reading the opcode and front end half of the instruction. This determines the size of the opcode.
    Escape(operation::Size),
    /// Synchronize execution among other processors.
    Synchronize,
    /// Hint to the processor that the branch is likely taken. If this is incorrect, it results in a pipeline flush and 
    /// a performance penalty. This will not cause the entire operation to fail on its own. 
    BranchLikelyTaken(bool),
    /// Repeat the current instruction based on a specific algorithm.
    Repeat(Repeat)
}

pub enum Direct {
    EscapeByte,
    EscapeWord, 
    
    Synchronize,
    
    BranchLikelyTakenTrue,
    BranchLikelyTakenFalse,

    RepeatFixed,
    RepeatUntilEqual
}