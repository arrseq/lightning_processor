use std::io;
use std::io::{Error, ErrorKind, Seek, SeekFrom};
use std::marker::PhantomData;

pub trait Read<T> {
    fn read(&mut self, output: &mut [T]) -> Result<u64, Error>;
    
    fn read_exact(&mut self, output: &mut [T]) -> Result<u64, Error> {
        match self.read(output) {
            Ok(length) => if length != output.len() as u64 { Err(Error::from(ErrorKind::UnexpectedEof)) } else { Ok(length) },
            Err(error) => Err(error)
        }
    }
    
    fn match_strings(&mut self, output: &mut Vec<T>, strings: &[&String]) -> Result<String, Error> {
        let st
    }
}

pub trait Peek<T> {
    fn peek(&self, output: &mut [T]) -> Result<u64, Error>;
}

#[derive(Debug, PartialEq)]
pub struct Cursor<T: AsRef<[D]>, D> {
    inner: T,
    position: u64,
    phantom_data: PhantomData<D>
}

impl<T, D> Cursor<T, D> where
    T: AsRef<[D]>
{
    pub fn new(source: T) -> Self {
        Self {
            inner: source,
            position: 0,
            phantom_data: PhantomData
        }
    }
}

impl<T, D> Seek for Cursor<T, D> where
    T: AsRef<[D]>
{
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match pos {
            SeekFrom::Current(offset) => self.position = (self.position as i64 + offset) as u64,
            SeekFrom::Start(offset) => self.position = offset,
            SeekFrom::End(offset) => self.position = (self.inner.as_ref().len() as u64 as i64 + offset) as u64
        }

        // TODO: Fix overflow bug.

        Ok(self.position)
    }
}

impl<T, D> Read<D> for Cursor<T, D> where
    T: AsRef<[D]>,
    D: Copy
{
    fn read(&mut self, output: &mut [D]) -> Result<u64, Error> {
        let mut moved = 0u64;

        for out_val in output {
            match self.inner.as_ref().get(self.position as usize) {
                Some(value) => *out_val = *value,
                None => break,
            }
            
            self.position += 1;
            moved += 1;
        }
        
        Ok(moved)
    }
}

impl<T, D> Peek<D> for Cursor<T, D> where
    T: AsRef<[D]>,
    D: Copy
{
    fn peek(&self, output: &mut [D]) -> Result<u64, Error> {
        let mut moved = 0u64;

        for out_val in output {
            match self.inner.as_ref().get(self.position as usize) {
                Some(value) => *out_val = *value,
                None => break,
            }

            moved += 1;
        }

        Ok(moved)
    }
}