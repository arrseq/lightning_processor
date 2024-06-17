#![feature(core_intrinsics)]

use std::intrinsics::black_box;
use std::io::Cursor;
use atln_processor::{number, Core, memory};
use atln_processor::instruction::{Driver0Encoding, Instruction};
use atln_processor::memory::Memory;
use atln_processor::number::Type;

fn main() {
    println!("MAX: {}", memory::PAGES_MAX);
    
    let rom_bytes = include_bytes!("./deadbeef.bin");
    let mut rom = Cursor::new(rom_bytes);

    let mut core = Core::new();
    let mut mem = Memory::from(Vec::new());
    
    // core.execute(&mut rom, &mut mem).unwrap();
    let mut executed = 0u64;
    loop {
        let i = match Instruction::new(&mut rom) {
            Ok(value) => value,
            Err(_) => break
        };
        
        black_box(i);
        
        executed += 1;
    }
    
    println!("Execution completed; Completed={}", executed);
}