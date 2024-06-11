use std::io::Cursor;
use atln_processor::{number, Core};
use atln_processor::instruction::{Driver0Encoding, Instruction};
use atln_processor::memory::Memory;

fn main() {
    let uuid = 1218646354314924054u64;
    println!("{}", u64::MAX - uuid);

    let rom_bytes = include_bytes!("./deadbeef.bin");
    let mut rom = Cursor::new(rom_bytes);

    let mut core = Core::new();
    let mut mem = Memory::from(Vec::new());

    core.execute(&mut rom, &mut mem).unwrap();
}
