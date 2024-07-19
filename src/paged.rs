use std::collections::HashMap;
use std::fs::rename;
use std::io;
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};
use thiserror::Error;

#[cfg(test)]
mod test;

pub type Mappings = Vec<(u64, u64)>;

#[derive(Debug, PartialEq)]
pub struct Paged<'a, Memory> {
    /// Mappings from page's page to physical page.
    pub mappings: Mappings,
    pub memory: &'a mut Memory,
    pub invalid_page_error: bool
}

/// The page mapping does not exist in the mappings.
#[derive(Debug, Error)]
#[error("Mapping for page not found")]
pub struct InvalidPageError;

impl<'a, Memory: Seek + 'a> Paged<'a, Memory> {
    pub const PAGE_ITEM_BITS: u8 = 12;
    pub const PAGE_ITEM_MASK: u64 = 0x0000_0000_0000_0FFF;
    
    pub const PAGE_BITS: u8 = 64 - Self::PAGE_ITEM_BITS;
    pub const PAGE_MASK: u64 = !Self::PAGE_ITEM_MASK;
    
    /// Extract the page code from an address.
    pub fn extract_page(address: u64) -> u64 {
        address >> Self::PAGE_ITEM_BITS
    }
    
    /// Extract the item code from an address.
    pub fn extract_item(address: u64) -> u64 {
        address & Self::PAGE_ITEM_MASK
    }

    pub fn page_to_layer(page: u64) -> u64 {
        page << Self::PAGE_ITEM_BITS
    }
    
    /// Translate an addresses page bits from a virtual page to a physical page.
    /// 
    /// # Result
    /// containing the translated address. If the translation does not exist for the particular page, then 
    /// [Err(InvalidPageError)] is returned.
    pub fn translate_address(&self, address: u64) -> Result<u64, InvalidPageError> {
        let page = Self::extract_page(address);
        let mapping = self.mappings
            .iter()
            .rev()
            .find(|entry| entry.0 == page)
            .ok_or(InvalidPageError)?.1;
        let physical_page_layer = mapping << Self::PAGE_ITEM_BITS;
        let item_layer = Self::extract_item(address);
        Ok(physical_page_layer | item_layer)
    }
    
    /// Ensure the address doesn't overflow.
    fn check_overflow(physical_address: u64, buffer_length: u64) -> io::Result<()> {
        physical_address.checked_add(buffer_length).ok_or(io::Error::new(ErrorKind::UnexpectedEof, "Buffer with stream position overflows"))?;
        Ok(())
    }
    
    fn operation_length(&mut self, physical_address: u64, remaining: u64) -> io::Result<u64> {
        let start = Self::extract_item(physical_address);
        let end = (remaining + start).min(Self::PAGE_ITEM_MASK + 1);

        let operation_length = end
            .checked_sub(start)
            .expect("Bug resulted in end being smaller than the start");

        let translated = self.translate_address(physical_address).map_err(|_| {
            self.invalid_page_error = true;
            io::Error::new(ErrorKind::UnexpectedEof, "Reached end of paged region. Page fault error.")
        })?;
        self.invalid_page_error = false;

        self.memory.seek(SeekFrom::Start(translated))?;
        Ok(operation_length)
    }
    
    fn prepare_next(physical_address: &mut u64, bytes_handled: &mut u64, buffer_length: u64, remaining: u64) {
        *bytes_handled = buffer_length - remaining;
        *physical_address = Self::page_to_layer(Self::extract_page(*physical_address) + 1);
    }
}

// FIXME: Rust compiler error forces "+ 'a". issue opened by x4exr on github. Should be removed when resolved.
impl<'a, Memory: Seek + Read + 'a> Read for Paged<'a, Memory> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let buffer_length = buf.len() as u64;
        let mut physical_address = self.memory.stream_position()?;
        let start_address = physical_address;
        let mut bytes_read: u64 = 0;
        let mut frame = [0u8; Self::PAGE_ITEM_MASK as usize + 1];
        let mut remaining = buf.len() as u64;

        Self::check_overflow(physical_address, buffer_length)?;
        
        loop {
            let operation_length = self.operation_length(physical_address, remaining)?;

            // Read
            let mut capped_reader = self.memory.take(operation_length);
            let received = capped_reader.read(&mut frame)? as u64;
            if received == 0 { break; }
            remaining -= received;

            // Write back to output buffer
            let received_data = &frame[0..operation_length as usize];
            buf[bytes_read as usize..(operation_length + bytes_read) as usize].copy_from_slice(received_data);

            Self::prepare_next(&mut physical_address, &mut bytes_read, buffer_length, remaining);
            if remaining == 0 { break; }
        }

        self.memory.seek(SeekFrom::Start(start_address + bytes_read))?;
        Ok(bytes_read as usize)
    }
}

impl<'a, Memory: Seek + Write + 'a> Write for Paged<'a, Memory> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut physical_address = self.memory.stream_position()?;
        let start_address = physical_address;
        let mut bytes_written: u64 = 0;
        let mut remaining = buf.len() as u64;
        let buffer_length = buf.len() as u64;

        Self::check_overflow(physical_address, buffer_length)?;

        loop {
            let operation_length = self.operation_length(physical_address, remaining)?;

            // Write data
            let write_frame = &buf[bytes_written as usize..(operation_length + bytes_written) as usize];
            let written = self.memory.write(write_frame)? as u64;
            if written == 0 { break; }
            remaining -= written;

            Self::prepare_next(&mut physical_address, &mut bytes_written, buffer_length, remaining);
            if remaining == 0 { break; }
        }

        self.memory.seek(SeekFrom::Start(start_address + bytes_written))?;
        Ok(bytes_written as usize)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.memory.flush()
    }
}

impl<'a, Memory: Seek> Seek for Paged<'a, Memory> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.memory.seek(pos)
    }
}