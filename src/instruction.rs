use crate::instruction::address::{AccessMode, Address};
use crate::num::{MaskedU32, MaskedU8};

pub mod address;
pub mod vector;
pub mod branch;
pub mod encoding;
mod register;
mod arithmetic;

pub type SegmentCode = MaskedU8<0x3>;
pub type LargeImmediate = MaskedU32<0x1FFFFF>;
pub type ScaleCode = MaskedU8<0x03>;

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
        segment: SegmentCode,
        immediate: LargeImmediate
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