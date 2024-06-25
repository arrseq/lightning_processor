use std::sync::{Arc, Mutex};

use atln_processor::memory::Memory;

pub struct System {
    pub memory: Arc<Mutex<Memory>>
}

impl System {
    pub fn new() -> Self {
        Self {
            memory: Arc::new(Mutex::new(Memory::from(vec![0u8; 100])))
        }
    }
}