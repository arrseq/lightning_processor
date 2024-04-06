use std::io::Cursor;

fn main() {
    let bytes = vec![0, 0, 1];
    let rom = Cursor::new(bytes);
}