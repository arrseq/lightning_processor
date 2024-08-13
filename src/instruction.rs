pub mod operation;

use crate::num::{MaskedU16, MaskedU32, MaskedU8};

pub type SegmentCode = MaskedU8<0x3>
pub type RegisterCode = MaskedU8<0xF>;
pub type FlagCode = MaskedU8<0x07>;
pub type BranchHintCode = MaskedU8<0x3>;
pub type OperandCode = MaskedU8<0x3>;
pub type VectorComponentCode = MaskedU8<0x3>;
pub const VECTOR_SIZE: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VectorComponentMapping {
    operand: OperandCode,
    components: [VectorComponentCode; 2]
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VectorComponentFlags {
    operand: OperandCode,
    negate: bool,
    zero: bool
}

pub type AddressImmediate = MaskedU32<0x1FFFF>;
pub type ScaleCode = MaskedU8<0x3>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddressMode {
    Absolute,
    Relative
}

pub type BaseOffset = MaskedU16<0x1FFF>;
pub type IndexedBaseOffset = MaskedU16<0x1FF>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IndexedBaseOffsetMode {
    Immediate(IndexedBaseOffset),
    Register(RegisterCode)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BaseMode {
    Offset(BaseOffset),
    RegisterOffset(RegisterCode),
    Indexed {
        index: RegisterCode,
        offset: IndexedBaseOffsetMode
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Address {
    Immediate {
        immediate: AddressImmediate,
        mode: AddressMode
    },
    Register {
        register: RegisterCode,
        mode: AddressMode
    },
    Base {
        base: RegisterCode, 
        mode: BaseMode
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    LoadImmediate {
        segment: SegmentCode,
        immediate: u16
    },
    DualSource {
        operation: operation::DualSource,
        source: [RegisterCode; 2]
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
    LoadVectorComponents {
        destination: RegisterCode,
        /// Having [None] means that the component corresponding to the index should be 0.
        components: [Option<VectorComponentCode>; VECTOR_SIZE]
    },
    ExtractVectorComponents {
        vector: RegisterCode,
        /// Having [None] means that the component corresponding to the index should not be extracted into a register.
        components: [Option<RegisterCode>; VECTOR_SIZE]
    },
    FlagVectorComponents {
        flags: [VectorComponentFlags; VECTOR_SIZE],
        temporary: bool
    },
    /// Only supports 2 operands due to the size constrain of an instruction.
    MapVector {
        mappings: [VectorComponentMapping; 2],
        temporary: bool
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
    },
    Branch {
        condition: FlagCode,
        hint: Option<bool>,
        address: Address
    }
}