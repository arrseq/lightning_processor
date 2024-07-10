use std::io::Read;
use arrseq_memory::dynamic_number;
use crate::operand::register::Register;

/// A tuple containing a register and a constant which will be operated on and then used to address memory.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Calculated {
    pub register: Register,
    pub base: dynamic_number::Unsigned
}

/// A dynamic address dereferencing source target. 
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddressTarget {
    /// Address memory with the dynamic operand's register field.
    Register(Register),
    
    /// Address memory with the instruction constant field.
    Constant(dynamic_number::Unsigned),
    
    /// Address memory with the sum of the dynamic operand's register field.
    Add(Calculated),
    
    /// Address memory with the difference of the dynamic operand's register field.
    Subtract(Calculated)
}

/// A dynamic source operand.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dynamic {
    Register(Register),
    Constant(dynamic_number::Unsigned),
    AddressTarget(AddressTarget)
}

impl Dynamic {
    pub fn encode(input: &mut impl Read) -> Self {
        todo!()
    }
}