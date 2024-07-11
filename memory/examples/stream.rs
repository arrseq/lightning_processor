extern crate arrseq_memory;

use arrseq_memory::stream::{Cursor, Read};

fn main() {
    let mut cursor: Cursor<Vec<u8>, u8> = Cursor::new(vec![230u8; 10]);
    
    let mut buffer = [0u8; 2];
    cursor.read(&mut buffer);
    
    dbg!(buffer);
}