use std::collections::HashMap;
use std::io;
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};
use crate::dynamic_number;

#[derive(Debug, PartialEq)]
pub struct Paged<'a, Memory> {
    /// Mappings from page's page to physical page.
    pub mappings: &'a HashMap<u64, u64>,
    pub memory: &'a mut Memory
}

#[derive(Debug)]
pub struct InvalidPageError;

impl<'a, Memory> Paged<'a, Memory> {
    pub const PAGE_ITEM_BITS: u8 = 12;
    pub const PAGE_BITS: u8 = 52;
    pub const PAGE_MASK: u64 = 0x0000_0000_0000_0FFF;
    
    /// Extract the page code from an address.
    pub fn extract_page(address: u64) -> u64 {
        address >> Self::PAGE_ITEM_BITS
    }
    
    /// Extract the item code from an address.
    pub fn extract_item(address: u64) -> u64 {
        address & Self::PAGE_MASK
    }
    
    /// Translate an addresses page bits from a virtual page to a physical page.
    /// 
    /// # Result
    /// containing the translated address. If the translation does not exist for the particular page, then 
    /// [Err(InvalidPageError)] is returned.
    /// 
    /// # Example
    /// ```
    /// use std::collections::HashMap;
    /// use std::io::Cursor;
    /// use arrseq_lightning::memory::Paged;
    /// 
    /// let mut mem = Cursor::new(vec![0u8; 1024]);
    /// let paged = Paged::new(HashMap::from([
    ///     (0xA, 0xB)
    /// ]), &mut mem);
    ///
    /// assert_eq!(paged.translate_address(0x00_00_00_00_00_00_A_F00).unwrap(), 0x00_00_00_00_00_00_B_F00); 
    /// ```
    pub fn translate_address(&self, address: u64) -> Result<u64, InvalidPageError> {
        let page = Self::extract_page(address);
        let mapping = *self.mappings.get(&page).ok_or(InvalidPageError)?;
        let physical_page_layer = mapping << Self::PAGE_ITEM_BITS;
        let item_layer = Self::extract_item(address);
        Ok(physical_page_layer | item_layer)
    }
}

impl<'a, Memory: Seek + Read> Read for Paged<'a, Memory> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let address = self.memory.stream_position()?;
        let translated_address = self.translate_address(address).map_err(|_| io::Error::new(ErrorKind::InvalidInput, "Invalid virtual address page."))?;
        
        // Read data then return stream position to original.
        self.memory.seek(SeekFrom::Start(translated_address))?;
        let result = self.memory.read(buf)?;
        self.memory.seek(SeekFrom::Start(address))?;
        
        Ok(result)
    }
}

impl<'a, Memory: Seek + Write> Write for Paged<'a, Memory> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let address = self.memory.stream_position()?;
        let translated_address = self.translate_address(address).map_err(|_| io::Error::new(ErrorKind::InvalidInput, "Invalid virtual address page."))?;

        self.memory.seek(SeekFrom::Start(translated_address))?;
        let result = self.memory.write(buf)?;
        self.memory.seek(SeekFrom::Start(address))?;

        Ok(result)
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

#[derive(Debug, PartialEq)]
pub struct Aligned<'a, Memory> {
    pub memory: &'a mut Memory,
    unaligned_access_error: bool
}

impl<'a, Memory> Aligned<'a, Memory> {
    pub fn new(memory: &'a mut Memory) -> Self {
        Self {
            memory,
            unaligned_access_error: false
        }
    }
    
    pub fn is_word_aligned(address: u64) -> bool {
        address % dynamic_number::Size::WORD_BYTES as u64 == 0
    }

    pub fn is_double_word_aligned(address: u64) -> bool {
        address % dynamic_number::Size::DOUBLE_WORD_BYTES as u64 == 0
    }

    pub fn is_quad_word_aligned(address: u64) -> bool {
        address % dynamic_number::Size::QUAD_WORD_BYTES as u64 == 0
    }
    
    pub fn is_aligned(address: u64, buffer_length: usize) -> Result<bool, dynamic_number::InvalidByteCountError> {
        Ok(match buffer_length {
            dynamic_number::Size::WORD_BYTES => Self::is_word_aligned(address),
            dynamic_number::Size::DOUBLE_WORD_BYTES => Self::is_double_word_aligned(address),
            dynamic_number::Size::QUAD_WORD_BYTES => Self::is_quad_word_aligned(address),
            _ => return Err(dynamic_number::InvalidByteCountError)
        })
    }
}

impl<'a, Memory: Seek + Read> Read for Aligned<'a, Memory> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Check if the address is unaligned.
        if Self::is_aligned(self.memory.stream_position()?, buf.len()).map_err(|_| io::Error::new(ErrorKind::InvalidInput, "Buffer size is not supported"))? {
            self.unaligned_access_error = true;
            return Err(io::Error::new(ErrorKind::InvalidInput, "The address was not aligned"));
        }
        
        self.unaligned_access_error = false;
        self.memory.read(buf)
    }
}