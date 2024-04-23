use std::{io::{Cursor, Read}, thread, time::Duration};

use emulator::{instruction::{self, Instruction, OperandsPresence, Operations}, memory::Memory};

fn main() {
    let mut core0 = emulator::core::Core::new(0);
    let mut ram = Memory::new(Some(10));

    loop {
        match core0.step(&mut ram) {
            Err(ecode) => match ecode {
                instruction::Errors::EndOfStream => break,
                instruction::Errors::OperationUnmatched => panic!("Improper format"),
                instruction::Errors::Seek(_) => panic!("THIS SHOULD NOT HAPPEN")
            },
            Ok(_) => println!("Successful step")
        }

        thread::sleep(Duration::from_millis(500));
    }

    println!("Graceful completion of core0 execution");
}