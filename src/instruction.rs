pub mod branch;
pub mod memory;
pub mod register;
pub mod flag;

use crate::num;
use flag::Flag;
use register::Register;

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
    Branch {
        operation: branch::Operation,
        condition: Option<Flag>, hint: branch::Hint,
        address: branch::Address },
    Memory {
        operation: memory::Operation,
        size: num::Size, register: Register,
        address: memory::Address }
}
