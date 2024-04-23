use std::io::{self, Read, Seek, Write};

use crate::{instruction::{self, Errors, Instruction}, register}; 

pub enum Permission {
    None,
    NonSafe,
    All
}

pub struct Core {
    registers: register::File,
    safe: bool,
    parser: instruction::Parser,
    instruction: Instruction
}

impl Core {
    pub fn new() -> Self {
        Core {
            safe: false,
            registers: register::File::new(),
            parser: instruction::Parser::new(),
            instruction: Instruction::default()
        }
    }

    pub fn step<Source: Read + Seek + Write>(&mut self, memory: &mut Source) -> Result<(), instruction::Errors> {
        match self.parser.parse(&mut self.instruction, memory) {
            Err(error) => return Err(error),
            Ok(_) => {}
        }

        println!("INSTRUCTION CODE: {}", self.instruction.operation);
        println!("-- Destination: {:?}", self.instruction.destination);
        println!("-- Source 0: {:?} -- Source 1: {:?}", self.instruction.source0, self.instruction.source1);

        match &self.instruction.immediate {
            None => println!("-- Immediate: None"),
            Some(imm) => println!("-- Immediate: {}", imm.into_qword())
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