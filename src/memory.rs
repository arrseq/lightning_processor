use std::collections::HashMap;
use std::io;
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct Paged<'a, Memory> {
    /// Mappings from page's page to physical page.
    pub mappings: HashMap<u64, u64>,
    pub memory: &'a mut Memory,
    pub invalid_page_error: bool
}

/// The page mapping does not exist in the mappings.
#[derive(Debug, Error)]
#[error("Mapping for page not found")]
pub struct InvalidPageError;

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
        let mut count = 0;
        let mut offset_address = address;
        let mut temporary_output = [0u8; 1];
        
        // Ensure the address doesn't overflow.
        address.checked_add(buf.len() as u64).ok_or(io::Error::new(ErrorKind::UnexpectedEof, "Buffer with stream position overflows"))?;
        
        for element in buf.iter_mut() {
            let translated_address = self.translate_address(offset_address).map_err(|_| {
                self.invalid_page_error = true;
                io::Error::new(ErrorKind::UnexpectedEof, "Invalid virtual address page. This does not mean you reached the end, there may be gap in the paging")
            })?;
            self.invalid_page_error = false;
            
            self.memory.seek(SeekFrom::Start(translated_address))?;
            if self.memory.read(&mut temporary_output)? == 0 { break; }
                        
            *element = temporary_output[0];
            
            // Starts at zero, so no chance for overflow.
            count += 1;
            
            // Safe to do because the overflow address was already checked.
            offset_address = address + count;
        }
        
        // To keep the illusion of this being seamless, this sets the position of the real stream position to what would 
        // be expected.
        self.memory.seek(SeekFrom::Start(offset_address))?;
        
        Ok(count as usize)
    }
}

impl<'a, Memory: Seek + Write> Write for Paged<'a, Memory> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        todo!()
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