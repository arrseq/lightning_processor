extern crate test;

use std::collections::HashMap;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use test::Bencher;
use crate::paged::{Paged, InvalidPageError};

#[bench]
fn benchmarked_read(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut memory = Cursor::new(vec![0u8; 16]);
        let mut paged = Paged {
            memory: &mut memory,
            mappings: HashMap::from([ (0, 0) ]),
            invalid_page_error: false
        };

        let mut buffer = test::black_box([0u8; 8]);
        paged.read_exact(&mut buffer).unwrap();
    });
}

#[bench]
fn benchmarked_large_read(bencher: &mut Bencher) {
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
fn benchmarked_write(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut memory = Cursor::new(vec![0u8; 16]);
        let mut paged = Paged {
            memory: &mut memory,
            mappings: HashMap::from([ (0, 0) ]),
            invalid_page_error: false
        };

        let buffer = [100u8; 8];
        paged.write_all(&buffer).unwrap();
    });
}

#[bench]
fn benchmarked_large_write(bencher: &mut Bencher) {
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

#[test]
fn translate_address() {
    let mut mem = Cursor::new(vec![0u8; 1024]);
    let paged = Paged {
        memory: &mut mem,
        mappings: HashMap::from([ (0xA, 0xB) ]),
        invalid_page_error: false
    };

    assert_eq!(paged.translate_address(0x0000_0000_0000_A_F00).unwrap(), 0x0000_0000_0000_B_F00);
    assert!(matches!(paged.translate_address(0x0000_0000_0000_F_F00).unwrap_err(), InvalidPageError));
}

#[test]
fn read() {
    let mut data = vec![100u8; 4096];
    data.extend(vec![200u8; 4096]);
    let mut memory = Cursor::new(data);
    let mut paged = Paged {
        memory: &mut memory,
        mappings: HashMap::from([ (0, 1), (1, 0) ]),
        invalid_page_error: false
    };

    let mut buffer = [0u8; 4096];

    paged.read_exact(&mut buffer).unwrap();
    assert_eq!(buffer, [200u8; 4096]);
    assert_eq!(paged.stream_position().unwrap(), 4096);

    paged.read_exact(&mut buffer).unwrap();
    assert_eq!(buffer, [100u8; 4096]);
    assert_eq!(paged.stream_position().unwrap(), 8192);

    // Data between pages
    paged.seek(SeekFrom::Start(2048)).unwrap();
    paged.read_exact(&mut buffer).unwrap();
    let mid_way_data = {
        let mut mid_way_data = vec![200u8; 2048];
        mid_way_data.extend(vec![100u8; 2048]);
        mid_way_data
    };
    assert_eq!(buffer, mid_way_data.as_slice());
    assert_eq!(paged.stream_position().unwrap(), 4096 + 2048);
}

#[test]
fn write() {
    let mut memory: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    let mut paged = Paged {
        memory: &mut memory,
        mappings: HashMap::from([ (0, 0) ]),
        invalid_page_error: false
    };

    let buffer = {
        let mut buffer = vec![200u8; 2048];
        buffer.extend(vec![100u8; 2048]);
        buffer
    };

    paged.write_all(&buffer).unwrap();
    assert_eq!(*paged.memory.get_ref(), buffer);
    assert_eq!(paged.stream_position().unwrap(), 4096);
}