//! If a prefix from the same category but different modes are used, then only the first instance of it is considered.

use std::io;
use std::io::{Read, Write};
use strum_macros::FromRepr;
use instruction::operation;
use number::low::LowSize;

#[derive(Debug, Clone, Copy)]
pub enum Repeat {
    /// Repeat this instruction a fixed number of times based on the value of the A register.
    Fixed,
    /// Repeat this instruction until the A register is equal to the B register.
    UntilEqual
}

#[derive(Debug, Clone, Copy)]
pub enum ExecutionMode {
    /// Synchronize execution among other processors.
    Synchronize,
    /// Repeat the current instruction based on a specific algorithm.
    Repeat(Repeat)
}

#[derive(Debug, Clone, Copy)]
pub struct Prefixes {
    /// Escape into reading the opcode and front end half of the instruction. This determines the size of the opcode.
    pub escape: LowSize,
    /// Set the instruction set code. This allows you to execute instructions from a different instruction set.
    pub extension: Option<operation::Extension>,
    /// Hint to the processor that the branch is likely taken. If this is incorrect, it results in a pipeline flush and
    /// a performance penalty. This will not cause the entire operation to fail on its own.
    pub branch_likely_taken: Option<bool>,
    pub execution_mode: Option<ExecutionMode>
}

#[derive(Debug, Clone, Copy)]
pub enum Prefix {
    Escape(LowSize),
    Extension(operation::Extension),
    BranchLikelyTaken(bool),
    ExecutionMode(ExecutionMode)
}

impl From<Code> for Prefix {
    fn from(value: Code) -> Self {
        match value {
            Code::EscapeByte => Self::Escape(LowSize::Byte),
            Code::EscapeWord => Self::Escape(LowSize::Word),
            Code::ExtensionBasic => Self::Extension(operation::Extension::Basic),
            Code::ExtensionFloating => Self::Extension(operation::Extension::Floating),
            Code::BranchLikelyTaken => Self::BranchLikelyTaken(true),
            Code::BranchNotLikelyTaken => Self::BranchLikelyTaken(false),
            Code::ExecutionModeSynchronize => Self::ExecutionMode(ExecutionMode::Synchronize),
            Code::ExecutionModeRepeatFixed => Self::ExecutionMode(ExecutionMode::Repeat(Repeat::Fixed)),
            Code::ExecutionModeRepeatUntilEqual => Self::ExecutionMode(ExecutionMode::Repeat(Repeat::UntilEqual))
        }
    }
}

#[derive(Copy, Clone, FromRepr)]
#[repr(u8)]
pub enum Code {
    EscapeByte,
    EscapeWord,
    
    ExtensionBasic,
    ExtensionFloating,
    
    BranchLikelyTaken,
    BranchNotLikelyTaken,
    
    ExecutionModeSynchronize,
    ExecutionModeRepeatFixed,
    ExecutionModeRepeatUntilEqual,
}

#[derive(Debug)]
pub enum DecodeError {
    ReadError(io::Error),
    /// An invalid prefix was used. This contains the invalid prefix that was encountered.
    InvalidPrefixError(u8)
}

impl Prefixes {
    pub fn encode<Output: Write + Extend<u8>>(&self, output: &mut Output) {
        if let Some(extension) = &self.extension {
            output.extend([ match extension {
                operation::Extension::Basic => Code::ExtensionBasic as u8,
                operation::Extension::Floating => Code::ExtensionFloating as u8
            } ])
        }

        if let Some(branch_likely_taken) = &self.branch_likely_taken { output.extend([ if *branch_likely_taken { Code::BranchLikelyTaken as u8 } else { Code::BranchNotLikelyTaken as u8 } ]); }

        if let Some(execution_mode) = &self.execution_mode {
            output.extend([ match execution_mode {
                ExecutionMode::Synchronize => Code::ExecutionModeSynchronize as u8,
                ExecutionMode::Repeat(repeat) => match repeat {
                    Repeat::Fixed => Code::ExecutionModeRepeatFixed as u8,
                    Repeat::UntilEqual => Code::ExecutionModeRepeatUntilEqual as u8
                }
            } ])
        }

        // The escape prefix must be encoded last as the processor will immediately start reading the instruction
        // front end after this.
        output.extend([ match self.escape {
            LowSize::Byte => Code::EscapeByte as u8,
            LowSize::Word => Code::EscapeWord as u8
        } ]);
    }

    pub fn decode<Input: Read>(input: &mut Input) -> Result<Self, DecodeError> {
        let mut buffer = [0u8; 1];
        let mut extension: Option<operation::Extension> = None;
        let mut branch_likely_taken: Option<bool> = None;
        let mut execution_mode: Option<ExecutionMode> = None;

        loop {
            input.read_exact(&mut buffer).map_err(DecodeError::ReadError)?;

            let prefix_id = buffer[0];
            let code = Code::from_repr(prefix_id).ok_or(DecodeError::InvalidPrefixError(prefix_id))?;
            let prefix = Prefix::from(code);

            match prefix {
                Prefix::Escape(escape) => return Ok(Self { escape, extension, branch_likely_taken, execution_mode }),
                Prefix::Extension(prefix_extension) => extension = Some(prefix_extension),
                Prefix::ExecutionMode(prefix_execution_mode) => execution_mode = Some(prefix_execution_mode),
                Prefix::BranchLikelyTaken(prefix_branch_likely_taken) => branch_likely_taken = Some(prefix_branch_likely_taken)
            }
        }
    }
}