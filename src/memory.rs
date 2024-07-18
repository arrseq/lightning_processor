use std::collections::HashMap;
use std::io;
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct Paged<'a, Memory> {
    /// Mappings from page's page to physical page.
    pub mappings: HashMap<u64, u64>,
    pub memory: &'a mut Memory
}

#[derive(Debug, Error)]
pub enum AddressTranslationError {
    /// The page mapping does not exist in the mappings.
    #[error("Mapping for page not found")]
    InvalidPage,
    
    /// The address and buffer length together overflow the page.
    #[error("Address and buffer length overflow page")]
    Overflow
}

impl<'a, Memory> Paged<'a, Memory> {
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
    /// use arrseq_lightning::memory::{AddressTranslationError, Paged};
    ///
    /// let mut mem = Cursor::new(vec![0u8; 1024]);
    /// let paged = Paged {
    ///     memory: &mut mem,
    ///     mappings: HashMap::from([
    ///         (0xA, 0xB)
    ///     ])
    /// };
    ///
    /// assert_eq!(paged.translate_address(0x0000_0000_0000_A_F00, 4).unwrap(), 0x0000_0000_0000_B_F00); 
    /// assert!(matches!(paged.translate_address(0x0000_0000_0000_F_F00, 4).unwrap_err(), AddressTranslationError::InvalidPage)); 
    ///
    /// // Test overflow.
    /// assert_eq!(paged.translate_address(0x0000_0000_0000_A_FFE, 1).unwrap(), 0x0000_0000_0000_B_FFE);
    /// 
    /// assert!(matches!(paged.translate_address(0x0000_0000_0000_A_FFF, 1).unwrap_err(), AddressTranslationError::Overflow));
    /// assert_eq!(paged.translate_address(0x0000_0000_0000_A_FFF, 0).unwrap(), 0x0000_0000_0000_B_FFF);
    /// ```
    pub fn translate_address(&self, address: u64, buf_len: u64) -> Result<u64, AddressTranslationError> {
        let page = Self::extract_page(address);
        let mapping = *self.mappings.get(&page).ok_or(AddressTranslationError::InvalidPage)?;
        let physical_page_layer = mapping << Self::PAGE_ITEM_BITS;
        let item_layer = Self::extract_item(address);
        
        // Check if the end address of the target (address and buffer length) does not overflow the page.
        let end_index = item_layer.checked_add(buf_len).ok_or(AddressTranslationError::Overflow)?;
        if end_index > Self::PAGE_ITEM_MASK { return Err(AddressTranslationError::Overflow); }
        
        Ok(physical_page_layer | item_layer)
    }
}

impl<'a, Memory: Seek + Read> Read for Paged<'a, Memory> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let address = self.memory.stream_position()?;
        let translated_address = self.translate_address(address, buf.len() as u64).map_err(|_| io::Error::new(ErrorKind::InvalidInput, "Invalid virtual address page."))?;
        
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
        let translated_address = self.translate_address(address, buf.len() as u64).map_err(|_| io::Error::new(ErrorKind::InvalidInput, "Invalid virtual address page."))?;

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