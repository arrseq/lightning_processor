use std::io::{self, Seek};

use crate::environment::register;

use super::{firmware::Entry, MicroInstruction};

pub struct Traverser {
    instruction_pointer: u8,
    entry: Entry
}

impl Traverser {
    pub fn new(entry: Entry) -> Self {
        Self {
            instruction_pointer: 0,
            entry
        }
    }
}

pub enum TraverserErrors {
    InvalidPointer,
    SeekError(io::Error)
}

impl Traverser {
    fn fix_pointer(&mut self) {
        let max = self.entry.instructions.len() as u8 - 1;
        if self.instruction_pointer > max {
            self.instruction_pointer = max;
        }
    }

    pub fn read(&mut self, target: &mut MicroInstruction, registers: &register::File) -> Result<(), TraverserErrors> {
        let micro_instruction = match self.entry.instructions.get(self.instruction_pointer as usize) {
            None => return Err(TraverserErrors::InvalidPointer),
            Some(instruction) => instruction
        };

        match micro_instruction.clone() {
            MicroInstruction::Divert { diversion_address } => self.instruction_pointer = diversion_address,
            MicroInstruction::DivertEqual { diversion_address, target, source } => {
                if registers.at_identifier(target).unwrap_or_default() >
                    registers.at_identifier(source).unwrap_or_default() {
                    self.instruction_pointer = diversion_address;
                }
            },
            // TODO
            _ => ()
        };

        *target = micro_instruction.clone();

        match self.seek(io::SeekFrom::Current(1)) {
            Err(error) => return Err(TraverserErrors::SeekError(error)),
            Ok(_) => ()
        };
        Ok(())
    }
}

impl Seek for Traverser {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        match pos {
            io::SeekFrom::Current(offset) => {
                let displacement =  offset.abs() as u8;

                if offset < 0 {
                    self.instruction_pointer -= displacement;
                } else {
                    self.instruction_pointer += displacement;
                }
            },
            io::SeekFrom::Start(offset) => {
                self.instruction_pointer += offset as u8;
            },
            io::SeekFrom::End(offset) => {
                self.instruction_pointer = self.entry.instructions.len() as u8 - 1 - (offset.abs() as u8);
            }
        }

        self.fix_pointer();
        Ok(self.instruction_pointer as u64)
    }
}
