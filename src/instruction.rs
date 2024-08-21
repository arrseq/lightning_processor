use proc_bitfield::{bitfield, ConvRaw};
use crate::num::MaskedU8;

pub mod memory;
pub mod mnemonic;
pub mod register;
pub mod unary;

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

    LoadImmediate(register::LoadImmediateOperation),
    BuildVector(register::BuildVectorOperation),
    UnBuildVector(register::UnBuildVectorOperation),
    
    ReadMemory(memory::ReadOperation),
    WriteMemory(memory::WriteOperation),
    Stack(memory::StackOperation),
    UnStack(memory::UnStackOperation),
    Lock(memory::LockOperation),
    Branch(memory::BranchOperation),
    
    Copy(register::CopyOperation),
    
    Unary(unary::Operation),
    Negate(unary::NegateOperation),
    ExtendSign(unary::ExtendSignOperation),
    
    Binary,
    RegroupingBinary,
    
    RegroupingQuaternary
}

impl Instruction {
    pub const MNEMONIC_CODE          : u8 = 0;
    pub const LOAD_IMMEDIATE_CODE    : u8 = 1;
    pub const BUILD_VECTOR_CODE      : u8 = 2;
    pub const UNBUILD_VECTOR_CODE    : u8 = 3;
    pub const READ_MEMORY_CODE       : u8 = 4;
    pub const WRITE_MEMORY_CODE      : u8 = 5;
    pub const STACK_CODE             : u8 = 6;
    pub const UNSTACK_CODE           : u8 = 7;
    pub const LOCK_CODE              : u8 = 8;
    pub const BRANCH_CODE            : u8 = 9;
    pub const COPY_CODE              : u8 = 10;
    pub const UNARY_CODE             : u8 = 11;
    pub const NEGATE_CODE            : u8 = 12;
    pub const EXTEND_SIGN_CODE       : u8 = 13;
    pub const BINARY_CODE            : u8 = 14;
    pub const REGROUP_BINARY_CODE    : u8 = 15;
    pub const REGROUP_QUATERNARY_CODE: u8 = 16;
    
    pub fn decode(encoded: u32) -> Self {
        let operation = Format::from(encoded).operation();
        match operation {
            Self::MNEMONIC_CODE => Self::Mnemonic(mnemonic::Format::from(encoded).operation()),
            Self::LOAD_IMMEDIATE_CODE => Self::LoadImmediate(register::LoadImmediateOperation::from(encoded)),
            Self::BUILD_VECTOR_CODE => Self::BuildVector(register::BuildVectorOperation::from(encoded)),
            Self::UNBUILD_VECTOR_CODE => Self::UnBuildVector(register::UnBuildVectorOperation::from(encoded)),
            Self::READ_MEMORY_CODE => Self::ReadMemory(memory::ReadOperation::from(encoded)),
            Self::WRITE_MEMORY_CODE => Self::WriteMemory(memory::WriteOperation::from(encoded)),
            Self::STACK_CODE => Self::Stack(memory::StackOperation::from(encoded)),
            Self::UNSTACK_CODE => Self::UnStack(memory::UnStackOperation::from(encoded)),
            Self::LOCK_CODE => Self::Lock(memory::LockOperation::from(encoded)),
            Self::BRANCH_CODE => Self::Branch(memory::BranchOperation::from(encoded)),
            Self::COPY_CODE => Self::Copy(register::CopyOperation::from(encoded)),
            Self::UNARY_CODE => Self::Unary(unary::Operation::from(encoded)),
            Self::NEGATE_CODE => Self::Negate(unary::NegateOperation::from(encoded)),
            Self::EXTEND_SIGN_CODE => Self::ExtendSign(unary::ExtendSignOperation::from(encoded)),

            _ => unimplemented!()
        }
    }
    
    pub fn encode(self) -> u32 {
        let (opcode, operands) = match self {
            Instruction::Mnemonic(op) => (Self::MNEMONIC_CODE, u32::from(op)),
            Instruction::LoadImmediate(op) => (Self::LOAD_IMMEDIATE_CODE, u32::from(op)),
            Instruction::BuildVector(op) => (Self::BUILD_VECTOR_CODE, u32::from(op)),
            Instruction::UnBuildVector(op) => (Self::UNBUILD_VECTOR_CODE, u32::from(op)),
            Instruction::ReadMemory(op) => (Self::READ_MEMORY_CODE, u32::from(op)),
            Instruction::WriteMemory(op) => (Self::WRITE_MEMORY_CODE, u32::from(op)),
            Instruction::Stack(op) => (Self::STACK_CODE, u32::from(op)),
            Instruction::UnStack(op) => (Self::UNSTACK_CODE, u32::from(op)),
            Instruction::Lock(op) => (Self::LOCK_CODE, u32::from(op)),
            Instruction::Branch(op) => (Self::BRANCH_CODE, u32::from(op)),
            Instruction::Copy(op) => (Self::COPY_CODE, u32::from(op)),
            Instruction::Unary(op) => (Self::UNARY_CODE, u32::from(op)),
            Instruction::Negate(op) => (Self::NEGATE_CODE, u32::from(op)),
            Instruction::ExtendSign(op) => (Self::EXTEND_SIGN_CODE, u32::from(op)),
            // Instruction::Binary(op) => (Self::BINARY_CODE, u32::from(op)),
            // Instruction::RegroupingBinary(op) => (Self::REGROUPING_BINARY_CODE, u32::from(op)),
            // Instruction::RegroupingQuaternary(op) => (Self::REGROUPING_QUATERNARY_CODE, u32::from(op)),
            _ => todo!()
        };
        
        let opcode = opcode as u32;
        opcode | operands
    }
}

impl Default for Instruction {
    fn default() -> Self {
        Self::Mnemonic(mnemonic::Operation::default())
    }
}