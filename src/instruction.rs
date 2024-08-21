use proc_bitfield::{bitfield, ConvRaw};
use crate::num::MaskedU8;

pub mod load_immediate;
pub mod memory;
pub mod mnemonic;
pub mod vector;

bitfield! {
    /// A bitfield showcasing how to specify bit ranges.
    #[derive(Clone, Copy, PartialEq, Eq)]
    struct Format(pub u32): Debug, FromRaw, IntoRaw { operation: u8 @ 0..=4 }
}

#[derive(Debug, Clone, Copy, PartialEq, ConvRaw)]
pub enum Scale { X8, X16, X32, X64 }

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Instruction {
    Mnemonic(mnemonic::Operation),

    LoadImmediate(load_immediate::Operation),
    BuildVector(vector::BuildVectorOperation),
    UnBuildVector(vector::UnBuildVectorOperation),
    
    ReadMemory(memory::ReadOperation),
    WriteMemory(memory::WriteOperation),
    Stack(memory::StackOperation),
    UnStack(memory::UnStackOperation),
    Lock(memory::LockOperation),
    Branch(memory::BranchOperation),
    
    Copy,
    
    Unary,
    Negate,
    SignExtend,
    Binary,
    RegroupingBinary,
    RegroupingQuaternary
}

impl Instruction {
    pub const MNEMONIC_CODE      : u8 = 0;
    pub const LOAD_IMMEDIATE_CODE: u8 = 1;
    pub const BUILD_VECTOR_CODE  : u8 = 2;
    pub const UNBUILD_VECTOR_CODE: u8 = 3;
    pub const READ_MEMORY_CODE   : u8 = 4;
    pub const WRITE_MEMORY_CODE  : u8 = 5;
    pub const STACK_CODE         : u8 = 6;
    pub const UNSTACK_CODE       : u8 = 7;
    pub const LOCK_CODE          : u8 = 8;
    pub const BRANCH_CODE        : u8 = 9;
    
    pub fn decode(encoded: u32) -> Self {
        let operation = Format::from(encoded);
        match operation.operation() {
            Self::MNEMONIC_CODE => Self::Mnemonic(mnemonic::Format::from(encoded).operation()),
            Self::LOAD_IMMEDIATE_CODE => Self::LoadImmediate(load_immediate::Operation::from(encoded)),
            Self::BUILD_VECTOR_CODE => Self::BuildVector(vector::BuildVectorOperation::from(encoded)),
            Self::UNBUILD_VECTOR_CODE => Self::UnBuildVector(vector::UnBuildVectorOperation::from(encoded)),
            Self::READ_MEMORY_CODE => Self::ReadMemory(memory::ReadOperation::from(encoded)),
            Self::WRITE_MEMORY_CODE => Self::WriteMemory(memory::WriteOperation::from(encoded)),
            Self::STACK_CODE => Self::Stack(memory::StackOperation::from(encoded)),
            Self::UNSTACK_CODE => Self::UnStack(memory::UnStackOperation::from(encoded)),
            Self::LOCK_CODE => Self::Lock(memory::LockOperation::from(encoded)),
            Self::BRANCH_CODE => Self::Branch(memory::BranchOperation::from(encoded)),

            _ => unimplemented!()
        }
    }
}

impl Default for Instruction {
    fn default() -> Self {
        Self::Mnemonic(mnemonic::Operation::default())
    }
}