extern crate atln_processor;

use atln_processor::number::{CheckedAdd, Number};

fn main() {
    let data = Number::Word(u16::MAX - 1);
    dbg!(data.checked_add(Number::Word(2)));
}