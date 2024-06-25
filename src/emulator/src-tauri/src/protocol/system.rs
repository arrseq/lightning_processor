use atln_processor::memory::Memory;

pub struct System {
    pub memory: Memory
}

impl System {
    pub fn new() -> Self {
        Self {
            memory: Memory::from(vec![0u8; 100])
        }
    }
}