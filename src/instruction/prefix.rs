//! If a prefix from the same category but different modes are used, then only the first instance of it is considered.

use instruction::operation;
use number::low::LowSize;
use utility::EncodeDynamic;

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

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Prefix {
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

impl EncodeDynamic for Prefixes {
    fn encode_dyn(&self, output: &mut Vec<u8>) {
        if let Some(extension) = &self.extension {
            output.push(match extension {
                operation::Extension::Basic => Prefix::ExtensionBasic as u8,
                operation::Extension::Floating => Prefix::ExtensionFloating as u8
            })
        }

        if let Some(branch_likely_taken) = &self.branch_likely_taken { output.push(if *branch_likely_taken { Prefix::BranchLikelyTaken as u8 } else { Prefix::BranchNotLikelyTaken as u8 }); }

        if let Some(execution_mode) = &self.execution_mode {
            output.push(match execution_mode {
                ExecutionMode::Synchronize => Prefix::ExecutionModeSynchronize as u8,
                ExecutionMode::Repeat(repeat) => match repeat {
                    Repeat::Fixed => Prefix::ExecutionModeRepeatFixed as u8,
                    Repeat::UntilEqual => Prefix::ExecutionModeRepeatUntilEqual as u8
                }
            })
        }

        // The escape prefix must be encoded last as the processor will immediately start reading the instruction
        // front end after this.
        output.push(match self.escape {
            LowSize::Byte => Prefix::EscapeByte as u8,
            LowSize::Word => Prefix::EscapeWord as u8
        });
    }
}