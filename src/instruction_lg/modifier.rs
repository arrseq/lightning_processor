use std::io;
use std::io::{Read, Write};
use thiserror::Error;

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
pub enum Modifier {
    Escape(Escape),
    Execution(Execution),
    
    /// Contains a hint to whether the branch is likely to be taken.
    BranchLikelyTaken(bool),
    SegmentedOperands
}

#[derive(Debug, Error)]
#[error("Invalid prefix code")]
pub struct InvalidCodeError;

impl Modifier {
    pub const BYTE_ESCAPE: u8 = 0;
    pub const WORD_ESCAPE: u8 = 1;
    pub const SYNCHRONIZED_EXECUTION: u8 = 2;
    pub const FIXED_REPEATING_EXECUTION: u8 = 3;
    pub const CONDITIONALLY_REPEATING_EXECUTION: u8 = 4;
    pub const DECREMENTING_REPEATED_EXECUTION: u8 = 5;
    pub const BRANCH_LIKELY_TAKEN: u8 = 6;
    pub const BRANCH_NOT_LIKELY_TAKEN: u8 = 7;
    pub const SEGMENTED_OPERANDS: u8 = 8;
    
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
            Self::SEGMENTED_OPERANDS => Self::SegmentedOperands,
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
            Self::BranchLikelyTaken(likely) => if likely { Self::BRANCH_LIKELY_TAKEN } else { Self::BRANCH_NOT_LIKELY_TAKEN },
            Self::SegmentedOperands => Self::SEGMENTED_OPERANDS
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Modifiers {
    pub escape: Escape,
    pub execution: Option<Execution>,
    pub branch_likely_taken: Option<bool>,
    pub segmented_operands: bool
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("")]
    Read(io::Error),
    
    #[error("")]
    InvalidCode(InvalidCodeError),
    
    #[error("Escape never provided")]
    MissingEscape
}

impl Modifiers {
    pub fn encode(self, output: &mut impl Write) -> Result<(), io::Error> {
        let mut buffer = [Modifier::Escape(self.escape).encode(); 1];
        output.write_all(&buffer)?;
        
        if let Some(execution) = self.execution {
            buffer[0] = Modifier::Execution(execution).encode();
            output.write_all(&buffer)?;
        }
        
        if let Some(likely) = self.branch_likely_taken {
            buffer[0] = Modifier::BranchLikelyTaken(likely).encode();
            output.write_all(&buffer)?;
        }
        
        Ok(())
    }
    
    pub fn decode(input: &mut impl Read) -> Result<Self, DecodeError> {
        let escape: Option<Escape>;
        let mut execution: Option<Execution> = None;
        let mut branch_likely_taken: Option<bool> = None;
        let mut segmented_operands = false;
        
        let mut buffer = [0u8; 1];
        loop {
            input.read_exact(&mut buffer).map_err(DecodeError::Read)?;
            let prefix = Modifier::decode(buffer[0]).map_err(DecodeError::InvalidCode)?;
            
            match prefix {
                Modifier::Escape(value) => break escape = Some(value),
                Modifier::Execution(value) => execution = Some(value),
                Modifier::BranchLikelyTaken(value) => branch_likely_taken = Some(value),
                Modifier::SegmentedOperands => segmented_operands = true
            }
        }
        
        if let Some(escape) = escape { return Ok(Self { escape, execution, branch_likely_taken, segmented_operands }); }
        Err(DecodeError::MissingEscape)
    }
}