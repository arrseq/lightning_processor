use std::io::{Cursor, Seek, SeekFrom};
use arrseq_lightning::read_cursor;
use arrseq_lightning::instruction::operation::Operation;
use arrseq_lightning::math::dynamic_number::Unsigned;

fn main() {
    let mut cursor = Cursor::new(vec![0u8; 0]);
    Unsigned::new(800).encode_chain(&mut cursor, true);

    dbg!(cursor.clone());
    cursor.seek(SeekFrom::Start(0)).unwrap();
    dbg!(Unsigned::decode_chain(&mut cursor, Some(4)));
}