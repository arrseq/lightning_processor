use std::io::{self, Read, Seek, Write};

use crate::{instruction::{self, Errors, Instruction, Operations}, memory::{self, Memory, RegisterCodes}}; 

pub enum Permission {
    None,
    NonSafe,
    All
}

pub struct Core {
    pub registers: memory::File,
    safe: bool,
    parser: instruction::Parser,
    instruction: Instruction,
}

impl Core {
    pub fn new(identity: u8) -> Self {
        let mut registers = memory::File::new();

        let _ = registers.find_mut(RegisterCodes::Core)
            .unwrap()
            .set_value(false, identity as usize);
        
        Core {
            safe: false,
            registers,
            parser: instruction::Parser::new(),
            instruction: Instruction::default(),
        }
    }

    pub fn step(&mut self, memory: &mut Memory) -> Result<(), instruction::Errors> {
        let current_instruction = self.registers.find(RegisterCodes::CurrentInstruction)
            .unwrap()
            .get_value(false)
            .unwrap();

        // Set the current instruction pointer every time because this core will be 
        // access by other cores too potentially.
        // 
        // This is a mainly 2 step operation, seek then access. This may appear to 
        // cause an issue because you may believe its mutex will be unlocked (if there is one) 
        // but that is not true, the mutex will unlock outside the function. 
        match memory.seek(io::SeekFrom::Start(current_instruction as u64)) {
            Err(io_error) => return Err(Errors::Seek(io_error)),
            Ok(_) => {}
        };

        match self.parser.parse(&mut self.instruction, memory) {
            Err(error) => return Err(error),
            Ok(_) => {}
        }

        if self.instruction.operation == Operations::Divert as u8 {
            println!("Jump detected");
            self.registers.find_mut(RegisterCodes::CurrentInstruction)
                .unwrap()
                .value = self.instruction.source0.unwrap() as usize;
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