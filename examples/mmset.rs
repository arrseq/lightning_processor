extern crate atln_processor;

use atln_processor::emulator::memory::{Frame, Memory};
use atln_processor::number::{Number, Size};

fn main() {
    let mut memory = Memory::from(vec![0u8; 1024]);
    
    memory.set(Frame {
        size: Size::Word,
        address: 2
    }, false, Number::Word(u16::MAX))
        .unwrap();
    
    dbg!(memory);   
}