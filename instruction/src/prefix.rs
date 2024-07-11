#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Escape {
    Byte,
    Word
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Repeat {
    Fixed,
    Condition,
    Decremental
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Execution {
    Synchronize,
    Repeat(Repeat)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Prefix {
    Escape(Escape),
    Execution(Execution),
    BranchLikelyTaken(bool)
}

#[derive(Debug)]
pub struct InvalidCodeError;

impl Prefix {
    pub const BYTE_ESCAPE: u8 = 0;
    pub const WORD_ESCAPE: u8 = 1;
    pub const SYNCHRONIZED_EXECUTION: u8 = 2;
    pub const FIXED_REPEATING_EXECUTION: u8 = 3;
    pub const CONDITIONALLY_REPEATING_EXECUTION: u8 = 4;
    pub const DECREMENTING_REPEATED_EXECUTION: u8 = 5;
    pub const BRANCH_LIKELY_TAKEN: u8 = 6;
    pub const BRANCH_NOT_LIKELY_TAKEN: u8 = 7;
    
    pub fn decode(encoded: u8) -> Result<Self, InvalidCodeError> {
        Ok(match encoded {
            Self::BYTE_ESCAPE => Self::Escape(Escape::Byte),
            Self::WORD_ESCAPE => Self::Escape(Escape::Word),
            Self::SYNCHRONIZED_EXECUTION => Self::Execution(Execution::Synchronize),
            Self::FIXED_REPEATING_EXECUTION => Self::Execution(Execution::Repeat(Repeat::Fixed)),
            Self::CONDITIONALLY_REPEATING_EXECUTION => Self::Execution(Execution::Repeat(Repeat::Condition)),
            Self::DECREMENTING_REPEATED_EXECUTION => Self::Execution(Execution::Repeat(Repeat::Decremental)),
            Self::BRANCH_LIKELY_TAKEN => Self::BranchLikelyTaken(true),
            Self::BRANCH_NOT_LIKELY_TAKEN => Self::BranchLikelyTaken(false),
            _ => return Err(InvalidCodeError)
        })
    }
    
    pub fn encode(self) -> u8 {
        match self {
            Self::Escape(escape) => match escape {
                Escape::Byte => Self::BYTE_ESCAPE,
                Escape::Word => Self::WORD_ESCAPE
            },
            Self::Execution(execution) => match execution {
                Execution::Synchronize => Self::SYNCHRONIZED_EXECUTION,
                Execution::Repeat(repeat) => match repeat {
                    Repeat::Fixed => Self::FIXED_REPEATING_EXECUTION,
                    Repeat::Condition => Self::CONDITIONALLY_REPEATING_EXECUTION,
                    Repeat::Decremental => Self::DECREMENTING_REPEATED_EXECUTION
                }
            },
            Self::BranchLikelyTaken(likely) => if likely { Self::BRANCH_LIKELY_TAKEN } else { Self::BRANCH_NOT_LIKELY_TAKEN }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Prefixes {
    pub escape: Escape,
    pub execution: Option<Execution>,
    pub branch_likely_taken: Option<bool>
}