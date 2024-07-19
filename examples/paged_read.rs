use std::collections::HashMap;
use std::io::{Cursor, Read, Seek, SeekFrom};
use arrseq_lightning::memory::Paged;

fn main() {
    let mut vector = vec![0u8; 8192];
    for el in vector.iter_mut().enumerate() {
        *el.1 = el.0 as u8
    }
    
    let mut cursor = Cursor::new(vector);
    let mut position_bounds = [0usize; 2];
    
    let mut paged = Paged {
        memory: &mut cursor,
        mappings: HashMap::from([
            (1, 0),
            (0, 1),
            (2, 2)
        ]),
        invalid_page_error: false
    };
    paged.seek(SeekFrom::Start(1)).expect("TODO: panic message");
    position_bounds[0] = paged.stream_position().unwrap() as usize;

    let mut buf = [0u8; 9000];
    // let mut buf = [0u8; 4096];
    
    let read = paged.read(&mut buf).unwrap();
    position_bounds[1] = cursor.stream_position().unwrap() as usize;
    
    dbg!(read, buf, position_bounds);
}