use crate::instruction::Format;
use crate::num::MaskedU8;

pub type Code = MaskedU8<0x7F>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DualSource {
    Compare
}

impl DualSource {
    pub const MAPPINGS: [(Code, DualSource); 1] = [(Code::new(0), Self::Compare)];
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Destination {
    Unstack
}

impl Destination {
    pub const MAPPINGS: [(Code, Destination); 1] = [(Code::new(1), Self::Unstack)];
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DestinationSource {
    CopyRegisterToRegister
}

impl DestinationSource {
    pub const MAPPINGS: [(Code, DestinationSource); 1] = [(Code::new(2), Self::CopyRegisterToRegister)];
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
    pub const MAPPINGS: [(Code, DestinationDualSource); 16] = [
        (Code::new(3), Self::Add),             (Code::new(4), Self::Subtract),             (Code::new(5), Self::Multiply),             (Code::new(6), Self::Divide),
        (Code::new(7), Self::AddFloat),        (Code::new(8), Self::SubtractFloat),        (Code::new(9), Self::MultiplyFloat),        (Code::new(10), Self::DivideFloat),
        (Code::new(11), Self::AddVector),      (Code::new(12), Self::SubtractVector),      (Code::new(13), Self::MultiplyVector),      (Code::new(14), Self::DivideVector),
        (Code::new(15), Self::AddFloatVector), (Code::new(16), Self::SubtractFloatVector), (Code::new(17), Self::MultiplyFloatVector), (Code::new(18), Self::DivideFloatVector),
    ];
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DestinationTripleSource {
    MultiplyAndAdd,
    AddAndMultiply

    // todo: Consider neccessity and operation space
}

impl DestinationTripleSource {
    pub const MAPPINGS: [(Code, DestinationTripleSource); 2] = [
        (Code::new(19), Self::MultiplyAndAdd), (Code::new(20), Self::AddAndMultiply),
    ];
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DualDestinationDualSource {
    DivideWithRemainder,
    DivideFloatWithRemainder,
    DivideVectorWithRemainder,
    DivideFloatVectorWithRemainder
}

impl DualDestinationDualSource {
    pub const MAPPINGS: [(Code, DualDestinationDualSource); 4] = [
        (Code::new(21), Self::DivideWithRemainder),       (Code::new(22), Self::DivideFloatWithRemainder),
        (Code::new(23), Self::DivideVectorWithRemainder), (Code::new(24), Self::DivideFloatVectorWithRemainder),
    ];
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Memory {
    Call,
    ReleaseMemory,
    Branch
}

impl Memory {
    pub const MAPPINGS: [(Code, Memory); 3] = [
        (Code::new(25), Self::Call), (Code::new(26), Self::ReleaseMemory), (Code::new(27), Self::Branch),
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
    pub const MAPPINGS: [(Code, SourceMemory); 8] = [
        (Code::new(28), Self::CopyMemoryByteToRegister),  (Code::new(29), Self::CopyMemoryWordToRegister),
        (Code::new(30), Self::CopyMemoryDwordToRegister), (Code::new(31), Self::CopyMemoryQwordToRegister),
        (Code::new(32), Self::AcquireMemoryByte),         (Code::new(33), Self::AcquireMemoryWord),
        (Code::new(34), Self::AcquireMemoryDword),        (Code::new(35), Self::AcquireMemoryQword),
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
    pub const MAPPINGS: [(Code, DestinationMemory); 4] = [
        (Code::new(36), Self::CopyRegisterByteToMemory),  (Code::new(37), Self::CopyRegisterWordToMemory),
        (Code::new(38), Self::CopyRegisterDwordToMemory), (Code::new(39), Self::CopyRegisterQwordToMemory),
    ];
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Entry<'a> {
    pub format: Format,
    pub name: &'a str
}

pub const MAPPINGS: [Entry; 47] = [
    Entry { format: Format::WaitForInterrupt,          name: "wait_for_interrupt" },
    Entry { format: Format::LoadImmediate,             name: "load_immediate" },
    Entry { format: Format::LoadVectorComponents,      name: "load_vector_components" },
    Entry { format: Format::ExtractVectorComponents,   name: "extract_vector_components" },
    Entry { format: Format::FlagVectorComponents,      name: "flag_vector_components" },
    Entry { format: Format::MapVector,                 name: "map_vector" },
    Entry { format: Format::Branch,                    name: "branch" },
    Entry { format: Format::DualSource,                name: "compare" },
    Entry { format: Format::Destination,               name: "unstack" },
    Entry { format: Format::DestinationSource,         name: "copy_register_to_register" },
    Entry { format: Format::DestinationDualSource,     name: "add" },
    Entry { format: Format::DestinationDualSource,     name: "subtract" },
    Entry { format: Format::DestinationDualSource,     name: "multiply" },
    Entry { format: Format::DestinationDualSource,     name: "divide" },
    Entry { format: Format::DestinationDualSource,     name: "add_float" },
    Entry { format: Format::DestinationDualSource,     name: "subtract_float" },
    Entry { format: Format::DestinationDualSource,     name: "multiply_float" },
    Entry { format: Format::DestinationDualSource,     name: "divide_float" },
    Entry { format: Format::DestinationDualSource,     name: "add_vector" },
    Entry { format: Format::DestinationDualSource,     name: "subtract_vector" },
    Entry { format: Format::DestinationDualSource,     name: "multiply_vector" },
    Entry { format: Format::DestinationDualSource,     name: "divide_vector" },
    Entry { format: Format::DestinationDualSource,     name: "add_float_vector" },
    Entry { format: Format::DestinationDualSource,     name: "subtract_float_vector" },
    Entry { format: Format::DestinationDualSource,     name: "multiply_float_vector" },
    Entry { format: Format::DestinationDualSource,     name: "divide_float_vector" },
    Entry { format: Format::DestinationTripleSource,   name: "multiply_and_add" },
    Entry { format: Format::DestinationTripleSource,   name: "add_and_multiply" },
    Entry { format: Format::DualDestinationDualSource, name: "divide_with_remainder" },
    Entry { format: Format::DualDestinationDualSource, name: "divide_float_with_remainder" },
    Entry { format: Format::DualDestinationDualSource, name: "divide_vector_with_remainder" },
    Entry { format: Format::DualDestinationDualSource, name: "divide_float_vector_with_remainder" },
    Entry { format: Format::Memory,                    name: "call" },
    Entry { format: Format::Memory,                    name: "release_memory" },
    Entry { format: Format::Memory,                    name: "branch" },
    Entry { format: Format::SourceMemory,              name: "copy_memory_byte_to_register" },
    Entry { format: Format::SourceMemory,              name: "copy_memory_word_to_register" },
    Entry { format: Format::SourceMemory,              name: "copy_memory_dword_to_register" },
    Entry { format: Format::SourceMemory,              name: "copy_memory_qword_to_register" },
    Entry { format: Format::SourceMemory,              name: "aquire_memory_byte" },
    Entry { format: Format::SourceMemory,              name: "aquire_memory_word" },
    Entry { format: Format::SourceMemory,              name: "aquire_memory_dword" },
    Entry { format: Format::SourceMemory,              name: "aquire_memory_qword" },
    Entry { format: Format::DestinationMemory,         name: "copy_register_byte_to_memory" },
    Entry { format: Format::DestinationMemory,         name: "copy_register_word_to_memory" },
    Entry { format: Format::DestinationMemory,         name: "copy_register_dword_to_memory" },
    Entry { format: Format::DestinationMemory,         name: "copy_register_qword_to_memory" }
];