use std::io::Cursor;

use architecture::instruction::{multi_sized_to_usize, Instruction, MultiSizedData, OperandsPresence, Parser};

enum Operation {
    LoadToRegister = 0
}

fn main() {
    // load to register
    let mut l2r = Parser::new(
        Operation::LoadToRegister as u8,
        OperandsPresence {
            destination: true,
            source0: false,
            source1: false,
            immediate: Some(MultiSizedData::QWord(0))
        }
    );

    // Program rom
    let rom_bytes = vec![
        0, 10, 00, 00, 00, 00, 00, 00, 16, 00, 
        0, 47, 26, 00, 00, 00, 00, 00, 16, 00
    ];
    let mut rom_cursor = Cursor::new(rom_bytes);

    let mut instruction = Instruction::default();
    
    for _ in 0..2 {
        match l2r.parse(&mut instruction, &mut rom_cursor) {
            Some(_) => return eprintln!("Failed to parse instruction L2R"),
            None => {}
        };
    
        println!("[Ok] Parsed instruction");
        println!("-- Operation: {}", instruction.operation);
        println!("-- Destination: {:?}", instruction.destination);
        println!("-- Source 0: {:?}", instruction.source0);
        println!("-- Source 1: {:?}", instruction.source1);
        
        if let Some(imm) = &instruction.immediate {
            println!("-- Immediate: {:?}", multi_sized_to_usize(imm.clone()))
        } else {
            return println!("o: Failed to parse immediate");
        }
    }
}