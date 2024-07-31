use std::io::Cursor;
use arrseq_lightning::cursor_test;
use arrseq_lightning::instruction::operation::Operation;
use arrseq_lightning::math::dynamic_number::Unsigned;

fn main() {
    dbg!(cursor_test([02], |x| Unsigned::decode_chain(x, Some(1))));
}