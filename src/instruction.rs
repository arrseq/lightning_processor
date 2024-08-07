use crate::num;
use crate::operation::branch;
use crate::state::flag::Flag;
use crate::state::register::Register;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegisterOperation {
    ReleaseMemory
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SizedRegisterOperation {
    
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DualSizedRegisterOperation {
    
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConditionalBranchOperation {

}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Nothing,
    WaitForInterrupt,
    Register { operation: RegisterOperation, register: Register },
    SizedRegister { 
        operation: SizedRegisterOperation, 
        size: num::Size, register: Register },
    DualSizedRegister {
        operation: DualSizedRegisterOperation,
        size: num::Size, registers: [Register; 2] },
    ConditionalBranch {
        operation: ConditionalBranchOperation,
        condition: Option<Flag>, hint: branch::Hint,
        address: branch::Address }
}