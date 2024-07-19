extern crate test;

use std::collections::HashMap;
use std::io;
use std::io::{Cursor, ErrorKind, Read, Seek, SeekFrom};
use test::Bencher;
use crate::memory::Paged;

#[bench]
fn byte_read(bencher: &mut Bencher) {
    fn read<'a, Memory: Seek + Read>(paged: &mut Paged<'a, Memory>, buf: &mut [u8]) -> io::Result<usize> {
        let address = paged.memory.stream_position()?;
        let mut count = 0;
        let mut offset_address = address;
        let mut temporary_output = [0u8; 1];

        // Ensure the address doesn't overflow.
        address.checked_add(buf.len() as u64).ok_or(io::Error::new(ErrorKind::UnexpectedEof, "Buffer with stream position overflows"))?;

        for element in buf.iter_mut() {
            let translated_address = paged.translate_address(offset_address).map_err(|_| {
                paged.invalid_page_error = true;
                io::Error::new(ErrorKind::UnexpectedEof, "Invalid virtual address page. This does not mean you reached the end, there may be gap in the paging")
            })?;
            paged.invalid_page_error = false;

            paged.memory.seek(SeekFrom::Start(translated_address))?;
            if paged.memory.read(&mut temporary_output)? == 0 { break; }

            *element = temporary_output[0];

            // Starts at zero, so no chance for overflow.
            count += 1;

            // Safe to do because the overflow address was already checked.
            offset_address = address + count;
        }

        // To keep the illusion of this being seamless, this sets the position of the real stream position to what would 
        // be expected.
        paged.memory.seek(SeekFrom::Start(offset_address))?;

        Ok(count as usize)
    }
    
    bencher.iter(|| {
        let mut memory = Cursor::new(vec![0u8; 8192]);
        let mut paged = Paged {
            memory: &mut memory,
            mappings: HashMap::from([
                (0, 1),
                (1, 0)
            ]),
            invalid_page_error: false
        };

        let mut buffer = test::black_box([0u8; 8192]);
        read(&mut paged, &mut buffer);
    });
}

#[bench]
fn framed_read(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut memory = Cursor::new(vec![0u8; 8192]);
        let mut paged = Paged {
            memory: &mut memory,
            mappings: HashMap::from([
                (0, 1),
                (1, 0)
            ]),
            invalid_page_error: false
        };
        
        let mut buffer = test::black_box([0u8; 8192]);
        paged.read_exact(&mut buffer).expect("Failed to read into buffer");
    });
}