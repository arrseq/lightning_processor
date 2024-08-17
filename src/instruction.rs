pub mod operation;
pub mod address;
pub mod vector;
pub mod flag;
pub mod encoding;

use crate::instruction::address::Address;
use crate::instruction::flag::Flag;
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
    MapVector,
    Branch,
    DualSource(operation::DualSource),
    Destination(operation::Destination),
    DestinationSource(operation::DestinationSource),
    DestinationDualSource(operation::DestinationDualSource),
    DestinationTripleSource(operation::DestinationTripleSource),
    DualDestinationDualSource(operation::DualDestinationDualSource),
    Memory(operation::Memory),
    SourceMemory(operation::SourceMemory),
    DestinationMemory(operation::DestinationMemory)
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
        components: [Option<RegisterCode>; vector::SIZE]
    },
    ExtractVectorComponents {
        source: RegisterCode,
        /// Having [None] means that the component corresponding to the index should not be extracted into a register.
        destinations: [Option<RegisterCode>; vector::SIZE]
    },
    /// Only supports 2 operands due to the size constrain of an instruction.
    MapVector {
        temporary: bool,
        operand: OperandCode,
        mappings: [vector::ComponentCode; vector::SIZE]
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