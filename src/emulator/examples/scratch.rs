use std::io::{Cursor, Read};

use emulator::{instruction::{self, Instruction, OperandsPresence, Operations}, memory::Memory};

fn main() {
    let mut core0 = emulator::core::Core::new();

    let mut buf = [0 as u8; 2];
    let mut ram = Memory::new(Some(10));

    for _ in 0..30 {
        match ram.read(&mut buf) {
            Err(_) => panic!("HUH"),
            Ok(len) => println!("Read {} bytes", len)
        }
        println!("{} {}", buf[0], buf[1]);
    }

    // loop {
    //     match core0.step(&mut ram) {
    //         Err(ecode) => match ecode {
    //             instruction::Errors::EndOfStream => break,
    //             instruction::Errors::OperationUnmatched => panic!("Improper format"),
    //             instruction::Errors::Seek(_) => panic!("THIS SHOULD NOT HAPPEN")
    //         },
    //         Ok(_) => println!("Successful step")
    //     }
    // }

    // println!("Graceful completion of core0 execution");
}