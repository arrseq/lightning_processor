use std::io::{self, Read, Seek, Write};

use crate::{instruction::{self, Errors, Instruction, Operations}, memory::{self, Memory}}; 

pub enum Permission {
    None,
    NonSafe,
    All
}

pub struct Core {
    pub registers: memory::File,
    safe: bool,
    parser: instruction::Parser,
    instruction: Instruction
}

impl Core {
    pub fn new() -> Self {
        Core {
            safe: false,
            registers: memory::File::new(),
            parser: instruction::Parser::new(),
            instruction: Instruction::default()
        }
    }

    pub fn step(&mut self, memory: &mut Memory) -> Result<(), instruction::Errors> {
        match self.parser.parse(&mut self.instruction, memory) {
            Err(error) => return Err(error),
            Ok(_) => {}
        }

        if self.instruction.operation == Operations::Divert as u8 {
            println!("Jump detected");
            memory.seek(io::SeekFrom::Start(0));
        }

        Ok(())
    }

    pub fn is_safe(&self) -> bool {
        self.safe
    }

    pub fn set_safe(&mut self, safe: bool) {
        // TODO: Implement
        todo!()
    }

    
}