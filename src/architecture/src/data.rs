use xp_common::traits::Mutable;

pub type ByteStreamResult = Result<u8, ()>;

pub trait ByteStream {
    // Get the next byte after the previous byte or initial byte.
    fn get_queued(&mut self) -> ByteStreamResult;

    // Get a byte relative to the current byte cursor by index.
    fn get_relative(&mut self, position: isize) -> ByteStreamResult;

    fn set_cursor(&mut self, cursor: usize);
    fn get_cursor(&mut self) -> usize;
    fn get_current(&mut self) -> ByteStreamResult;
    fn get_at(&mut self, point: usize) -> ByteStreamResult;

    fn get_bunch(&mut self, count: usize) -> Vec<ByteStreamResult> {
        let mut bunch: Vec<ByteStreamResult> = Vec::new();
        for _ in 0..count {
            bunch.push(self.get_queued());
        }
        bunch
    }
}

pub struct BytesAtomic {
    bytes: Vec<u8>
}

impl Mutable for BytesAtomic {
    type Type = Vec<u8>;

    fn get(&mut self) -> Self::Type {
        self.bytes.clone()
    }

    fn set(&mut self, value: Self::Type) {
        self.bytes = value;
    }
}

impl BytesAtomic {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes
        }
    }
}