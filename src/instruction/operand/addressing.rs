#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConstantBasedMode {
    Constant,
    ArrayInObject
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConstantBasedAddressing {
    pub mode: ConstantBasedMode,
    pub constant: u64,
    pub mask: u64
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Register,
    Array,
    ConstantBased(ConstantBasedMode)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Addressing {
    pub mode: Mode,
    pub index_register: u8
}