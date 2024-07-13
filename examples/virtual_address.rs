extern crate arrseq_lightning;

use std::collections::HashMap;
use std::io::{Cursor, Read};
use arrseq_lightning::memory::Paged;

fn main() {
    let mut memory = Cursor::new(vec![0u8; 4096 * 2]);

    {
        let mut_mem = memory.get_mut();
        mut_mem[4096] = 200;
        mut_mem[0] = 10;
    }
    
    let mut paged = Paged::new(HashMap::from([
        (0, 1)
    ]), &mut memory);
    
    let mut buf = [0u8; 1];
    paged.read_exact(&mut buf);
    
    dbg!(buf[0]);
}