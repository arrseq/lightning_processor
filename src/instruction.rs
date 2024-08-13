pub mod operation;
pub mod address;
pub mod vector;
pub mod flag;
pub mod encoding;

use crate::instruction::address::Address;
use crate::instruction::flag::Flag;
use crate::instruction::vector::{VectorComponentFlags, VectorComponentMapping};
use crate::num::{MaskedU8};

pub type SegmentCode = MaskedU8<0x3>;
pub type RegisterCode = MaskedU8<0xF>;
pub type BranchHintCode = MaskedU8<0x3>;
pub type OperandCode = MaskedU8<0x3>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Format {
    WaitForInterrupt,
    LoadImmediate,
    LoadVectorComponents,
    ExtractVectorComponents,
    FlagVectorComponents,
    MapVector,
    Branch,
    
    DualSource,
    Destination,
    DestinationSource,
    DestinationDualSource,
    DestinationTripleSource,
    DualDestinationDualSource,
    Memory,
    SourceMemory,
    DestinationMemory
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    WaitForInterrupt,
    LoadImmediate {
        destination: RegisterCode,
        segment: SegmentCode,
        immediate: u16
    },
    LoadVectorComponents {
        destination: RegisterCode,
        /// Having [None] means that the component corresponding to the index should be 0.
        components: [Option<vector::ComponentCode>; vector::SIZE]
    },
    ExtractVectorComponents {
        vector: RegisterCode,
        /// Having [None] means that the component corresponding to the index should not be extracted into a register.
        components: [Option<RegisterCode>; vector::SIZE]
    },
    FlagVectorComponents {
        flags: [VectorComponentFlags; vector::SIZE],
        temporary: bool
    },
    /// Only supports 2 operands due to the size constrain of an instruction.
    MapVector {
        mappings: [VectorComponentMapping; 2],
        temporary: bool
    },
    Branch {
        condition: Flag,
        hint: Option<bool>,
        address: Address
    },
    
    DualSource {
        operation: operation::DualSource,
        sources: [RegisterCode; 2]
    },
    Destination {
        operation: operation::Destination,
        destination: RegisterCode
    },
    DestinationSource {
        operation: operation::DestinationSource,
        destination: RegisterCode,
        source: RegisterCode
    },
    DestinationDualSource {
        operation: operation::DestinationDualSource,
        destination: RegisterCode,
        sources: [RegisterCode; 2]
    },
    DestinationTripleSource {
        operation: operation::DestinationTripleSource,
        destination: RegisterCode,
        sources: [RegisterCode; 3]
    },
    DualDestinationDualSource {
        operation: operation::DualDestinationDualSource,
        destinations: [RegisterCode; 2], 
        sources: [RegisterCode; 2]
    },
    Memory {
        operation: operation::Memory,
        address: Address
    },
    SourceMemory {
        operation: operation::SourceMemory,
        destination: RegisterCode,
        source: Address
    },
    DestinationMemory {
        operation: operation::DestinationMemory,
        destination: Address,
        source: RegisterCode
    }
}