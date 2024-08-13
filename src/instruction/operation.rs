//! Enums of all operations for specific operand formats.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DualSource {
    Compare
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Destination {
    Unstack
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DestinationSource {
    CopyRegisterToRegister
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DestinationDualSource {
    Add,
    Subtract,
    Multiply,
    Divide,

    AddFloat,
    SubtractFloat,
    MultiplyFloat,
    DivideFloat,

    AddVector,
    SubtractVector,
    MultiplyVector,
    DivideVector,

    AddFloatVector,
    SubtractFloatVector,
    MultiplyFloatVector,
    DivideFloatVector,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DestinationTripleSource {
    MultiplyAndAdd,
    AddAndMultiply
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DualDestinationDualSource {
    DivideWithRemainder,
    DivideFloatingWithRemainder,
    DivideVectorWithRemainder,
    DivideFloatVectorWithRemainder
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Memory {
    Call,
    ReleaseMemory,
    Branch
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SourceMemory {
    CopyMemoryByteToRegister,
    CopyMemoryWordToRegister,
    CopyMemoryDwordToRegister,
    CopyMemoryQwordToRegister,

    AcquireMemoryByte,
    AcquireMemoryWord,
    AcquireMemoryDword,
    AcquireMemoryQword
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DestinationMemory {
    CopyRegisterByteToMemory,
    CopyRegisterWordToMemory,
    CopyRegisterDwordToMemory,
    CopyRegisterQwordToMemory
}