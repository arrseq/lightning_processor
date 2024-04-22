use std::io::Cursor;

use emulator::instruction::{self, Instruction, Operations};

fn main() {
    let mut core0 = emulator::core::Core::new();

    core0.perform_register();
    core0.perform_register();

    let parser = instruction::Parser::new();
    let mut rom = {
        let inner: Vec<u8> = vec![Operations::LoadImmediateByte as u8, 0, 10, Operations::Safe as u8, 0, Operations::Add as u8, 0, 1, 2];
        Cursor::new(inner)
    };

    let mut ins = Instruction::default();
    loop {
        match parser.parse(&mut ins, &mut rom) {
            Err(variant) => match variant {
                instruction::Errors::EndOfStream => break,
                instruction::Errors::OperationUnmatched => panic!("Invalid operation"),
                instruction::Errors::Seek(_) => panic!("An IO error occurred")
            },
            Ok(_) => {}
        };
    
        println!("INS CODE: {:?}", ins.operation);
        println!("Dest Reg: {:?}", ins.destination);
        println!("S0 P: {:?}", ins.source0);
        println!("S1 P: {:?}", ins.source1);
        
        let imm = match ins.immediate {
            None => 0,
            Some(ref dy) => match &dy {
                instruction::MultiSizedData::Byte(u) => *u as u64,
                instruction::MultiSizedData::Word(u) => *u as u64,
                instruction::MultiSizedData::DWord(u) => *u as u64,
                instruction::MultiSizedData::QWord(u) => *u as u64,
            }
        };

        println!("Imm: {}", imm);
    }

    println!("Parsing complete");
}