use std::sync::{Arc, Mutex};
use atln_processor::emulator::memory::Memory;

pub struct System {
    pub memory: Arc<Mutex<Memory<Vec<u8>>>>
}

impl System {
    pub fn new() -> Self {
        Self {
            memory: Arc::new(Mutex::new(Memory::new(vec![100u8; 100])))
        }
    }
}