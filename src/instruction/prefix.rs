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

impl From<Direct> for Prefix {
    fn from(value: Direct) -> Self {
        match value {
            Direct::EscapeByte => Self::Escape(operation::Size::Byte),
            Direct::EscapeWord => Self::Escape(operation::Size::Word),
            
            Direct::Synchronize => Self::Synchronize,
            
            Direct::BranchLikelyTaken => Self::BranchLikelyTaken(true),
            Direct::BranchNotLikelyTaken => Self::BranchLikelyTaken(false),
            
            Direct::RepeatFixed => Self::Repeat(Repeat::Fixed),
            Direct::RepeatUntilEqual => Self::Repeat(Repeat::UntilEqual)
        }
    }
}

pub enum Direct {
    EscapeByte,
    EscapeWord, 
    
    Synchronize,

    BranchLikelyTaken,
    BranchNotLikelyTaken,

    RepeatFixed,
    RepeatUntilEqual
}

impl From<Prefix> for Direct {
    fn from(value: Prefix) -> Self {
        match value {
            Prefix::Escape(escape) => match escape {
                operation::Size::Byte => Self::EscapeByte,
                operation::Size::Word => Self::EscapeWord
            },
            Prefix::Synchronize => Self::Synchronize,
            Prefix::BranchLikelyTaken(likely_taken) => if likely_taken { Self::BranchLikelyTaken } else { Self::BranchNotLikelyTaken },
            Prefix::Repeat(algorithm) => match algorithm {
                Repeat::Fixed => Self::RepeatFixed,
                Repeat::UntilEqual => Self::RepeatUntilEqual
            }
        }
    }
}