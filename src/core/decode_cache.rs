use std::io;
use std::io::{Read, Seek};
use thiserror::Error;
use crate::instruction_lg;
use crate::instruction_lg::Instruction;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Entry {
    pub base_address: u64,
    pub instruction: Instruction,
    pub lifetime: usize
}

/// The cache should be cleared when a context switch occurs.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DecodeCache {
    pub decoded: Vec<Entry>,
    pub initial_lifetime: usize,
    pub chunk_size: usize
}

#[derive(Debug, Error)]
pub enum PopulateError {
    #[error("")]
    DecodeError(instruction_lg::DecodeError),
    
    #[error("Could not obtain the stream position")]
    Io(io::Error)
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
    
    /// Age all lifetimes by 1. Any that become 0 will be deleted.
    /// 
    /// # Result
    /// The number of entries pruned.
    pub fn age(&mut self) -> usize {
        self.decoded
            .iter_mut()
            .for_each(|entry| entry.lifetime -= 1);
        self.prune()
    }
    
    /// Consume a cached instruction and cache it.
    pub fn take(&mut self, base_address: u64) -> Option<Instruction> {
        self.age();
        let entry = self.decoded
            .iter_mut()
            .find(|entry| entry.base_address == base_address)?;
        entry.lifetime = self.initial_lifetime;
        Some(entry.instruction)
    }
    
    pub fn append(&mut self, base_address: u64, instruction: Instruction) {
        self.age();
        self.decoded.push(Entry {
            instruction, base_address,
            lifetime: self.initial_lifetime
        });
    }
    
    /// Populate the decoded cache and get the number of instructions cached.
    pub fn populate<Memory: Seek + Read>(&mut self, memory: &mut Memory) -> Result<usize, PopulateError> {
        let mut remaining = self.chunk_size.saturating_sub(self.decoded.len());
        let start_length = remaining;
        
        for _ in 0..start_length {
            let base_address = memory.stream_position().map_err(PopulateError::Io)?;
            let instruction = Instruction::decode(memory).map_err(PopulateError::DecodeError)?;
            self.append(base_address, instruction);
            remaining -= 1;
        }
        
        Ok(start_length - remaining)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Manager {
    pub cache: DecodeCache,
    /// How many ticks should happen before the cache attempts to repopulate itself.
    pub population_tick_interval: usize,
    current_tick: usize
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TickResult {
    pub did_populate: bool,
    pub instruction_count: usize
}

impl Manager {
    pub fn new(cache: DecodeCache, population_tick_interval: usize) -> Self {
        Self { cache, population_tick_interval, current_tick: 0 }
    }
    
    /// Execute a tick, once the population interval is reached, the cache will try to populate.
    /// 
    /// # Result
    /// If [Err(_)] is returned, this means the tick loop attempted to populate the cache but failed. If [Ok(_)] is 
    /// returned, then you must check the 'did_populate' field in the tick result.
    /// 
    /// If the cache was not to be populated, then the instruction count will also be 0.
    pub fn tick<Memory: Seek + Read>(&mut self, memory: &mut Memory) -> Result<TickResult, PopulateError> {
        self.current_tick += 1;
        if self.current_tick < self.population_tick_interval { return Ok(TickResult { did_populate: false, instruction_count: 0 }); }
        self.current_tick = 0;
        
        Ok(TickResult {
            did_populate: true,
            instruction_count: self.cache.populate(memory)?
        })
    }
}