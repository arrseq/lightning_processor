#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Destination {
    Unstack
}

impl Destination {
    pub const MAPPINGS: [Destination; 1] = [Self::Unstack];
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DestinationSource {
    CopyRegisterToRegister
}

impl DestinationSource {
    pub const MAPPINGS: [DestinationSource; 1] = [Self::CopyRegisterToRegister];
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

impl DestinationDualSource {
    pub const MAPPINGS: [DestinationDualSource; 16] = [
        Self::Add, Self::Subtract, Self::Multiply, Self::Divide,
        Self::AddFloat, Self::SubtractFloat, Self::MultiplyFloat, Self::DivideFloat,
        Self::AddVector, Self::SubtractVector, Self::MultiplyVector, Self::DivideVector,
        Self::AddFloatVector, Self::SubtractFloatVector, Self::MultiplyFloatVector, Self::DivideFloatVector,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DestinationTripleSource {
    MultiplyAndAdd,
    AddAndMultiply
}

impl DestinationTripleSource {
    pub const MAPPINGS: [DestinationTripleSource; 2] = [
        Self::MultiplyAndAdd, Self::AddAndMultiply,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DualDestinationDualSource {
    DivideWithRemainder,
    DivideFloatingWithRemainder,
    DivideVectorWithRemainder,
    DivideFloatVectorWithRemainder
}

impl DualDestinationDualSource {
    pub const MAPPINGS: [DualDestinationDualSource; 4] = [
        Self::DivideWithRemainder, Self::DivideFloatingWithRemainder,
        Self::DivideVectorWithRemainder, Self::DivideFloatVectorWithRemainder,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Memory {
    Call,
    ReleaseMemory,
    Branch
}

impl Memory {
    pub const MAPPINGS: [Memory; 3] = [
        Self::Call, Self::ReleaseMemory, Self::Branch,
    ];
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

impl SourceMemory {
    pub const MAPPINGS: [SourceMemory; 8] = [
        Self::CopyMemoryByteToRegister, Self::CopyMemoryWordToRegister,
        Self::CopyMemoryDwordToRegister, Self::CopyMemoryQwordToRegister,
        Self::AcquireMemoryByte, Self::AcquireMemoryWord,
        Self::AcquireMemoryDword, Self::AcquireMemoryQword,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DestinationMemory {
    CopyRegisterByteToMemory,
    CopyRegisterWordToMemory,
    CopyRegisterDwordToMemory,
    CopyRegisterQwordToMemory
}

impl DestinationMemory {
    pub const MAPPINGS: [DestinationMemory; 4] = [
        Self::CopyRegisterByteToMemory, Self::CopyRegisterWordToMemory,
        Self::CopyRegisterDwordToMemory, Self::CopyRegisterQwordToMemory,
    ];
}
