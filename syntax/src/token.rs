use std::fmt::Debug;
use arrseq_memory::stream::Cursor;
use crate::file::Range;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token<Key> {
    pub key: Key,
    pub range: Range
}

pub trait Processor: Debug + Clone + Copy + PartialEq {
    fn process<T: AsRef<[char]>>(self, input: &mut Cursor<T, char>);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tokenizer {
    
}