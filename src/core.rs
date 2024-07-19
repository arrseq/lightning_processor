use std::collections::HashMap;
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use crate::instruction;
use crate::instruction::Instruction;
use crate::instruction::operand::register;
use crate::paged::{Mappings, Paged};

pub mod decode_cache;

#[derive(Debug, Clone, PartialEq)]
pub enum Privilege {
    High,
    Low
}

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    pub instruction_pointer: u64,
    pub privilege: Privilege,
    pub registers: register::Collection,
    pub page_mappings: Option<Mappings>
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
            Privilege::Low => {
                let mut paged = Paged { 
                    memory: input,
                    mappings: self.context.page_mappings.take().unwrap(), // TODO: Maybe not a good idea
                    invalid_page_error: false
                };
                
                let decoded = Instruction::decode(&mut paged).map_err(DecodeError::Instruction);
                self.context.page_mappings = Some(paged.mappings);
                
                decoded?
            }
        };
        self.context.instruction_pointer = input.stream_position().map_err(DecodeError::Read)?;
        
        Ok(result)
    }
    
    pub fn execute(&mut self, _instruction: Instruction) -> Result<(), ExecuteError>{
        todo!()
    }
}

impl Default for Core {
    fn default() -> Self {
        Self {
            context: Context {
                instruction_pointer: 0,
                privilege: Privilege::High,
                registers: register::Collection::default(),
                page_mappings: Some(vec![
                    (0, 0),
                    (1, 1)
                ])
            }
        }
    }
}