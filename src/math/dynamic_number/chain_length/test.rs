use crate::cursor_test;
use crate::math::dynamic_number::Unsigned;

#[test]
fn decode() {
    assert_eq!(cursor_test([255, 254], |cursor| Unsigned::decode_chain_length(cursor, None).unwrap()), Unsigned::U16(254 + 254));
}

#[test]
fn decode_with_limit() {
    assert_eq!(cursor_test([10], |cursor| Unsigned::decode_chain_length(cursor, Some(0)).unwrap()), Unsigned::U8(0));
    assert_eq!(cursor_test([10], |cursor| Unsigned::decode_chain_length(cursor, Some(1)).unwrap()), Unsigned::U8(10));
    
    // Test to see if the last byte read is allowed to represent [u8::MAX].
    assert_eq!(cursor_test([254], |cursor| Unsigned::decode_chain_length(cursor, Some(1)).unwrap()), Unsigned::U8(254));
    assert_eq!(cursor_test([255], |cursor| Unsigned::decode_chain_length(cursor, Some(1)).unwrap()), Unsigned::U8(255));

    assert_eq!(cursor_test([255, 255], |cursor| Unsigned::decode_chain_length(cursor, Some(2)).unwrap()), Unsigned::U16(254 + 255));
}