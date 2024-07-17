use std::collections::HashMap;
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use crate::instruction;
use crate::instruction::Instruction;
use crate::instruction::operand::register;
use crate::memory::Paged;

pub mod decode_cache;

#[derive(Debug, Clone, PartialEq)]
pub enum Privilege {
    High,
    Low(HashMap<u64, u64>)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    pub instruction_pointer: u64,
    pub privilege: Privilege,
    pub registers: register::Collection
}

#[derive(Debug, Clone, PartialEq)]
pub struct Core {
    pub context: Context
}

#[derive(Debug)]
pub enum ExecuteError {
}

#[derive(Debug)]
pub enum DecodeError {
    Read(io::Error),
    Instruction(instruction::DecodeError)
}

impl Core {
    pub fn decode<Input: Read + Seek + Write>(&mut self, input: &mut Input) -> Result<Instruction, DecodeError> {
        input.seek(SeekFrom::Start(self.context.instruction_pointer)).map_err(DecodeError::Read)?;
        let result = match &self.context.privilege {
            Privilege::High => Instruction::decode(input).map_err(DecodeError::Instruction)?,
            Privilege::Low(mappings) => {
                let mut paged = Paged { mappings, memory: input };
                Instruction::decode(&mut paged).map_err(DecodeError::Instruction)?
            }
        };
        self.context.instruction_pointer = input.stream_position().map_err(DecodeError::Read)?;
        
        Ok(result)
    }
    
    pub fn execute(&mut self, instruction: Instruction) -> Result<(), ExecuteError>{
        todo!()
    }
}

impl Default for Core {
    fn default() -> Self {
        Self {
            context: Context {
                instruction_pointer: 0,
                privilege: Privilege::High,
                registers: register::Collection::default()
            }
        }
    }
}