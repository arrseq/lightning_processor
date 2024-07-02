extern crate atln_processor;

use atln_processor::emulator::memory::Memory;
use atln_processor::emulator::processor::processor::{Core, Ports};
use atln_processor::emulator::processor::processor::instruction::{Data, Instruction};
use atln_processor::emulator::processor::processor::instruction::operand::{AllPresent, Destination, Dynamic, Operands};
use atln_processor::emulator::processor::processor::instruction::operation::arithmetic::Arithmetic;
use atln_processor::emulator::processor::processor::instruction::operation::Extension;
use atln_processor::number::Size;

fn main() {
    let mut cpu0 = Core::default();
    let mut memory = Memory::from(vec![0u8; 1024]);
    let mut ports = Ports::default();

    let instruction = Instruction::new(Extension::Arithmetic(Arithmetic::Add), Some(Data {
        destination: Destination::Static,
        width: Size::Byte,
        synchronous: false,
        operands: Operands::AllPresent(AllPresent {
            x_static: 0,
            x_dynamic: Dynamic::Register(0)
        })
    })).unwrap();
    
    cpu0.execute(&instruction, &mut memory, &mut ports);
}