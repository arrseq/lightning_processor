use std::io::Cursor;
use arrseq_lightning::math::dynamic_number::DynamicNumber;

fn main() {
    let mut cursor = Cursor::new([255, 255, 0]);
    assert_eq!(DynamicNumber::U16(508), DynamicNumber::decode_unprefixed(&mut cursor).unwrap());
}