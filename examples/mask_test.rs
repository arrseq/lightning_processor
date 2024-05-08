//! Often times people dont know how to use bitwise operations in high-level programming. I wrote this file simply to
//! give people an idea on how bit masking works.

fn main() {
	// The goal is to extract the last 4 bits. My old method of doing this was to split the byte into bits with a 
	// function I found on stackoverflow and modified. The legacy code can be found in the `utility.rs` file. Don't 
	// use it. The code won't be deleted for literally no logical reason. I just insist on keeping it.
	let collection_dir = 0b00010101u8;
	let collection = collection_dir >> 1;
	let dir = collection_dir & 0b00000001;

	println!("{:08b} {}", collection, collection);
	println!("{:08b} {}", dir, dir);
}
