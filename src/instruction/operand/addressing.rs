#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConstantBasedAddressingMode {
    Constant,
    ArrayInObject
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConstantBasedAddressing {
    pub mode: ConstantBasedAddressingMode,
    pub constant: u64,
    pub mask: u64
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NonConstantBasedAddressingMode {
    Register,
    Array
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddressingMode {
    ConstantBasedAddressingMode(ConstantBasedAddressingMode),
    NonConstantBasedAddressingMode(NonConstantBasedAddressingMode)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Addressing {
    pub mode: AddressingMode,
    pub index_register: u8
}