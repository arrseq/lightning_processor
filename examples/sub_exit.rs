use std::io::Cursor;
use arrseq_lightning::instruction::Instruction;
use arrseq_lightning::instruction::operand::{AddressingMode, Operand};
use arrseq_lightning::instruction::operation::{DestinationAndDualInput, DualInput, Operation};
use arrseq_lightning::math::dynamic_number::{Size, Unsigned};

const PROGRAM: [Instruction; 2] = [
    // Decrement the counter.
    Instruction::new(&Operation::DestinationAndDualInput { 
        operation: DestinationAndDualInput::Subtract,
        destination: Operand::new_register(1, Size::X64),
        inputs: [
            Operand::new_register(1, Size::X64),
            Operand::new_value(Unsigned::new(1), Size::X64)
        ]
    }),
    
    // if the counter is 0 then jump to address 0x00.
    Instruction::new(&Operation::DualInput {
        operation: DualInput::Compare,
        inputs: [
            Operand::new_value(Unsigned::new(0), Size::X64),
            Operand::new_register(1, Size::X64)
        ]
    }),
    
    // FIXME: This program is incomplete and will not pass a test.
    // FIXME: The following should be implemented to meet completion. This requires a jump, halt and zero test
    // FIXME: instruction.
    // FIXME: - Check if register 1 is 0.
    // FIXME: - If zero flag is set, then jump to 0x0000000000000000 address. For simplicity, relative addressing 
    // FIXME:   wasn't used.
    // FIXME: - Halt execution. 
];

fn make_program() -> Cursor<Vec<u8>> {
    let mut cursor = Cursor::new(vec![0u8; 0]);
    for instruction in PROGRAM {
        // TODO: Implement encode().
    }
    cursor
}

/// A generator for a program that the emulator can run. This will generate code that subtracts a counter stored in 
/// system memory.
/// 
/// # Behavior
/// Counter starts from register 1 and exit is called when the counter reaches 0.
fn main() {
    let binary = make_program();
    dbg!(binary);
    
    dbg!(size_of::<Instruction>());
}