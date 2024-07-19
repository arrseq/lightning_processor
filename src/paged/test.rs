extern crate test;

use std::collections::HashMap;
use std::io::{Cursor, Read, Write};
use test::Bencher;

use crate::paged::Paged;

#[bench]
fn large_read(bencher: &mut Bencher) {
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
        paged.read_exact(&mut buffer).unwrap();
    });
}

#[bench]
fn large_write(bencher: &mut Bencher) {
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

        let buffer = [100u8; 8192];
        paged.write_all(&buffer).unwrap();
    });
}