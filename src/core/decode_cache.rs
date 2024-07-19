use std::io;
use std::io::{Read, Seek};
use crate::instruction::Instruction;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Entry {
    pub base_address: u64,
    pub instruction: Instruction,
    pub lifetime: usize
}

/// The cache should be cleared when a context switch occurs.
#[derive(Debug, Clone, PartialEq)]
pub struct DecodeCache {
    pub decoded: Vec<Entry>,
    pub initial_lifetime: usize
}

impl DecodeCache {
    pub fn find(&self, base_address: u64) -> Option<&Instruction> {
        self.decoded
            .iter()
            .find(|entry| entry.base_address == base_address && entry.lifetime != 0)
            .map(|e| &e.instruction)
    }
    
    /// Remove any entries with a lifetime of 0. 
    /// 
    /// # Result
    /// The number of entries removed.
    pub fn prune(&mut self) -> usize {
        let original_length = self.decoded.len();
        self.decoded.retain(|entry| entry.lifetime != 0);
        original_length - self.decoded.len()
    }
    
    /// Consume a cached instruction and cache it.
    pub fn take(&mut self, base_address: u64) -> Option<Instruction> {
        self.prune();
        let index = self.decoded
            .iter()
            .position(|entry| entry.base_address == base_address && entry.lifetime != 0)?;
        Some(self.decoded.remove(index).instruction)
    }
    
    /// Populate the decoded cache and get the number of instructions cached.
    pub fn populate<Memory: Seek + Read>(&mut self, memory: &mut Memory) -> io::Result<usize> {
        todo!()
    }
}