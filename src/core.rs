use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use thiserror::Error;
use crate::core::decode_cache::DecodeCache;
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
    pub page_mappings: Option<Mappings>,
    
    /// None signifies that there will be no decoded instruction caching.
    pub decode_cache: Option<DecodeCache>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Core {
    pub context: Context
}

#[derive(Debug)]
pub enum ExecuteError {
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("")]
    Read(io::Error),
    
    #[error("")]
    Instruction(instruction::DecodeError),
    
    /// The mappings vector wasn't set and it may have been taken.
    #[error("The mappings vector is None")]
    MappingsUnavailable
}

impl Core {
    pub fn decode<Input: Read + Seek + Write>(&mut self, input: &mut Input) -> Result<Instruction, DecodeError> {
        input.seek(SeekFrom::Start(self.context.instruction_pointer)).map_err(DecodeError::Read)?;
        let result = match &self.context.privilege {
            Privilege::High => Instruction::decode(input).map_err(DecodeError::Instruction)?,
            Privilege::Low => {
                let mut paged = Paged { 
                    memory: input,
                    mappings: self.context.page_mappings.take().ok_or(DecodeError::MappingsUnavailable)?,
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
                page_mappings: None,
                decode_cache: Some(DecodeCache { decoded: Vec::new(), initial_lifetime: 2, chunk_size: 10 })
            }
        }
    }
}