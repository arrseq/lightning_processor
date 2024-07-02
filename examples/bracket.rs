extern crate atln_processor;

use atln_processor::utility::{Bracket, FromRepresentation};

fn main() {
    let bracket = Bracket::from_representation("[".into());
    dbg!(bracket);
}