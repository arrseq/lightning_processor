use crate::math::dynamic_number::Unsigned;
use crate::write_cursor;

#[test]
fn encode() {
    // todo: write tests.
    dbg!(write_cursor(vec![0u8; 0], |cursor| Unsigned::new(255).encode_chain(cursor, Some(2))));
}