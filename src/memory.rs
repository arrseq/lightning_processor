use std::collections::HashMap;
use std::io;
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};
use crate::dynamic_number;

#[derive(Debug, PartialEq)]
pub struct Paged<'a, Memory: Seek> {
    /// Mappings from page's page to physical page.
    pub mappings: HashMap<u64, u64>,
    memory: &'a mut Memory
}

#[derive(Debug)]
pub struct InvalidPageError;

impl<'a, Memory: Seek> Paged<'a, Memory> {
    pub const PAGE_ITEM_BITS: u8 = 12;
    pub const PAGE_BITS: u8 = 52;
    pub const PAGE_MASK: u64 = 0x0000_0000_0000_0FFF;
    
    pub fn new(mappings: HashMap<u64, u64>, memory: &'a mut Memory) -> Self {
        Self { mappings, memory }
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

impl<'a, Memory: Seek> Seek for Paged<'a, Memory> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let (base, offset) = match pos {
            SeekFrom::Start(pos) => (pos, 0),
            SeekFrom::Current(pos) => (self.memory.stream_position()?, pos),
            SeekFrom::End(pos) => (self.memory.stream_len()?, pos)
        };
        
        let address = base.checked_add_signed(offset).ok_or(io::Error::new(ErrorKind::InvalidInput, "Invalid address overflow"))?;
        let translated = self.translate_address(address).map_err(|_| io::Error::new(ErrorKind::InvalidInput, "Invalid address after translation"))?;
        
        self.memory.seek(SeekFrom::Start(translated))
    }
}