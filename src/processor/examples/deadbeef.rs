#![feature(core_intrinsics)]

use atln_processor::Core;
use atln_processor::memory::Memory;

fn main() {
    let mut memory = Memory::default();
    let mut core = Core::default();
    
    core.decode(&mut memory).expect("Failed to decode from instruction stream");
}