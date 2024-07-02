extern crate atln_processor;

use atln_processor::number::{CheckedAdd, Data};

fn main() {
    let data = Data::Word(u16::MAX - 1);
    dbg!(data.checked_add(Data::Word(2)));
}