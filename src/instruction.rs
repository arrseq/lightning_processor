use crate::num::MaskedU8;

pub mod address;
pub mod vector;
pub mod branch;
pub mod encoding;
pub mod register;
pub mod arithmetic;
pub mod load_immediate;

pub type SegmentCode = MaskedU8<0x3>;
pub type ScaleCode = MaskedU8<0x03>;

pub const STACK_SOURCE_MASK: (u8, u32) = (0, 0b0000000000000000000000011111);
pub const UNSTACK_DEST_MASK: (u8, u32) = (0, 0b0000000000000000000000011111);

pub const LOAD_IMMEDIATE_DESTINATION_MASK: (u8, u32) = (0, 0b0000000000000000000000011111);
pub const LOAD_IMMEDIATE_SEGMENT_MASK    : (u8, u32) = (5, 0b0000000000000000000001100000);
pub const LOAD_IMMEDIATE_OFFSET_MASK     : (u8, u32) = (7, 0b1111111111111111111110000000);

pub const BUILD_VECTOR_DESTINATION_MASK: (u8, u32) = (0,  0b0000000000000000000000011111);
pub const BUILD_VECTOR_COMPONENT_0_MASK: (u8, u32) = (5,  0b0000000000000000001111100000);
pub const BUILD_VECTOR_COMPONENT_1_MASK: (u8, u32) = (10, 0b0000000000000111110000000000);
pub const BUILD_VECTOR_COMPONENT_2_MASK: (u8, u32) = (15, 0b0000000011111000000000000000);
pub const BUILD_VECTOR_COMPONENT_3_MASK: (u8, u32) = (20, 0b0001111100000000000000000000);

pub const UNBUILD_VECTOR_SOURCE_MASK       : (u8, u32) = (0,  0b0000000000000000000000011111);
pub const UNBUILD_VECTOR_DESTINATION_0_MASK: (u8, u32) = (5,  0b0000000000000000001111100000);
pub const UNBUILD_VECTOR_DESTINATION_1_MASK: (u8, u32) = (10, 0b0000000000000111110000000000);
pub const UNBUILD_VECTOR_DESTINATION_2_MASK: (u8, u32) = (15, 0b0000000011111000000000000000);
pub const UNBUILD_VECTOR_DESTINATION_3_MASK: (u8, u32) = (20, 0b0001111100000000000000000000);

pub const COPY_REGISTER_DESTINATION_FILE_MASK: (u8, u32) = (0,  0b0000000000000000000000000011);
pub const COPY_REGISTER_DESTINATION_MASK     : (u8, u32) = (0,  0b0000000000000000000001111100);
pub const COPY_REGISTER_SOURCE_FILE_MASK     : (u8, u32) = (0,  0b0000000000000000000110000000);
pub const COPY_REGISTER_SOURCE_MASK          : (u8, u32) = (0,  0b0000000000000011111000000000);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    None,
    Wait,
    
    End,
    EndInterrupt,
    Interrupt,
    
    Stack { source: register::Code },
    Unstack { destination: register::Code },
    
    LoadImmediate { 
        destination: register::Code,
        segment: load_immediate::Segment
    },
    BuildVector {
        destination: register::Code,
        components: [register::Code; vector::SIZE]
    },
    UnBuildVector {
        source: register::Code,
        destinations: [register::Code; vector::SIZE]
    },
    
    CopyRegister {
        destination: register::Code,
        destination_file: register::FileName,
        source: register::Code,
        source_file: register::FileName
    },
    
    Arithmetic {
        operation: arithmetic::Operation,
        vector: bool,
        atomic: bool,
        destination: register::Code,
        sources: [register::Code; 2]
    },
    Address {
        operation: address::Operation,
        data: address::Meta,
        offset: address::LargeOffset
    },
    AddressWithBase {
        operation: address::OperationWithBase,
        data: address::Meta,
        base: register::Code,
        offset: address::SmallOffset
    },
    Branch {
        operation: branch::Operation,
        data: branch::Meta,
        offset: branch::LargeOffset
    },
    BranchWithBase {
        operation: branch::OperationWithBase,
        data: branch::Meta,
        base: register::Code,
        offset: branch::SmallOffset
    },
    
    Timer,
    
    Lock,
    UnLock,
    
    Free2,
    Free3,
    Free4
}