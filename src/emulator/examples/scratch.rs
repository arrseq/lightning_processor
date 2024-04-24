use std::io::Cursor;

use emulator::binary;

fn main() {
    let bin = [0 as u8, 4, 2, 8, 2, 9, 3, 5];
    let mut stream = Cursor::new(bin);
    let mut reader = binary::Traverser::new(&mut stream);

    loop {
        let res = match reader.read_word() {
            None => break,
            Some(value) => value
        };

        println!("Res: {:?}", res);
    }
}

