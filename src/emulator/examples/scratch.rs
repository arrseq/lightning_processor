use std::io::Cursor;

use emulator::instruction::{self, Instruction, OperandsPresence, Operations};

fn main() {
    let mut core0 = emulator::core::Core::new();

    let mut ram = {
        let inner: Vec<u8> = vec![
            Operations::LoadImmediateByte as u8, 14, 10, 
            Operations::Safe as u8, 0, 
            Operations::Add as u8, 0, 1, 2, 
            Operations::Nothing as u8
        ];
        Cursor::new(inner)
    };

    loop {
        match core0.step(&mut ram) {
            Err(ecode) => match ecode {
                instruction::Errors::EndOfStream => break,
                instruction::Errors::OperationUnmatched => panic!("Improper format"),
                instruction::Errors::Seek(_) => panic!("THIS SHOULD NOT HAPPEN")
            },
            Ok(_) => println!("Successful execution")
        }
    }

    println!("Graceful completion of core0 execution");
}