use std::io::Cursor;
use atln_processor::{number, Core};
use atln_processor::memory::Memory;

fn main() {
	let rom_bytes = include_bytes!("../../../programs/rom.bin");
	let mut rom = Cursor::new(rom_bytes);

	let mut core = Core::new();
	let mut mem = Memory::from_size(16);

	core.execute(&mut rom, &mut mem).unwrap();
}
