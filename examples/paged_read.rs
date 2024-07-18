use std::collections::HashMap;
use std::io::{Cursor, Read, Seek};
use arrseq_lightning::memory::Paged;

fn main() {
    let mut cursor = Cursor::new(vec![100u8; 4096]);
    cursor.get_mut().extend(vec![200u8; 4096]);
    
    let mut position_bounds = [0usize; 2];
    position_bounds[0] = cursor.stream_position().unwrap() as usize;
    
    let mut paged = Paged {
        memory: &mut cursor,
        mappings: HashMap::from([
            (0, 1),
            (1, 0)
        ]),
        invalid_page_error: false
    };
    
    let mut buf = [0u8; 8192];
    paged.read_exact(&mut buf).expect("Failed to read");
    position_bounds[1] = cursor.stream_position().unwrap() as usize;
    
    dbg!(position_bounds);
}