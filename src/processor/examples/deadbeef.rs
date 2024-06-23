#![feature(core_intrinsics)]

use std::collections::HashMap;
use std::fs::{File, write};
use std::io::Write;
use std::path::Path;
use atln_processor::Core;
use atln_processor::memory::{Frame, Memory, PAGE_BYTES_COUNT};
use atln_processor::number::{Data, Size};

fn generate_html_memory(bytes: &Vec<u8>) -> String {
    let mut html = String::from("<DOCTYPE HTML><html><head><title>Memory Dump</title><style>tr:hover { background: rgba(0, 0, 0, 5%); }</style></head><body><table><tr><th>Dec Address</th><th>Dec Value</th><th>Bin Address</th><th>Bin Value</th></tr>");
    for index in 0..bytes.len() { html += &format!("<tr><td>{}</td><td>{}</td><td>{:064b}</td><td>{:08b}</td></tr>", index, bytes[index], index, bytes[index]); }
    html += "</table></body></html>";
    html
}

fn main() {
    // let mut memory = Memory::default();
    // let mut core = Core::default();
    // 
    // core.decode(&mut memory).expect("Failed to decode from instruction stream");

    use atln_processor::memory::{Frame, Memory, PAGE_BYTES_COUNT};
    use atln_processor::number::{Data, Size};
    
    let mut memory = Memory::from({
        let mut store = vec![0u8; (PAGE_BYTES_COUNT * 2) as usize];
      
        // Memory addresses are zero indexed.
        let second_page_index = PAGE_BYTES_COUNT as usize;

        store[second_page_index] = 255;
        store[second_page_index + 1] = 1;
    
        // To account for memory alignment.
        store[second_page_index + 5] = 1;
        store[second_page_index + 6] = 255;
        
        store
    });

    //
    let html = generate_html_memory(&memory.bytes);
    write("C:/tmp/emu_mem.html", html).expect("Could not write mem dump. C:/tmp/emu_mem.html");
    //
    
    // Map addresses from first virtual page boundary to the second hardware page. Hardware and virtual pages align
    // parallel.
    memory.pages.insert(0, 1);
    
    // Test.
    assert_eq!(memory.get(Frame { address: 0, size: Size::Byte }, true).unwrap(), Data::Byte(255));
    assert_eq!(memory.get(Frame { address: 0, size: Size::Word }, true).unwrap(), Data::Word(511));
    assert_eq!(memory.get(Frame { address: 4, size: Size::Word }, true).unwrap(), Data::Word(256));
    // endregion
}