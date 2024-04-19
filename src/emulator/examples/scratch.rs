use std::io::Cursor;

use emulator::instruction::{self, Instruction};

fn main() {
    // let mut core0 = emulator::core::Core::new();

    // core0.perform_register();
    let parser = instruction::Parser::new();
    let mut rom = {
        let inner: Vec<u8> = vec![0, 1, 2, 1];
        Cursor::new(inner)
    };

    let mut ins = Instruction::default();
    loop {
        let mut end = false;
        match parser.parse(&mut ins, &mut rom) {
            Err(variant) => match variant {
                instruction::Errors::EndOfStream => end = true,
                instruction::Errors::OperationUnmatched => panic!("Invalid operation"),
                instruction::Errors::Seek(_) => panic!("An IO error occurred")
            },
            Ok(_) => {}
        };
    
        println!("INS CODE: {:?}", ins.operation);
        if end {
            break;
        }
    }

    println!("Parsing complete");
}