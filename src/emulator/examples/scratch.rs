use std::io::Cursor;

use emulator::instruction::{self, Instruction, Operations};

fn main() {
    // let mut core0 = emulator::core::Core::new();

    // core0.perform_register();
    let parser = instruction::Parser::new();
    let mut rom = {
        let inner: Vec<u8> = vec![Operations::LoadImmediateByte as u8, 0, 10, Operations::Safe as u8, 0];
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
    }

    println!("Parsing complete");
}