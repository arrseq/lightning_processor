use std::collections::HashMap;
use std::fs::rename;
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

#[derive(Debug, Clone, Copy, PartialEq)]
struct OperationContext {
    
}

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
    
    fn operate(&mut self, target: &[u8], action: impl Fn(OperationContext) -> io::Result<()>) -> io::Result<usize> {
        let mut address = self.memory.stream_position()?;
        let mut bytes_read = 0;
        let mut frame = [0u8; Self::PAGE_ITEM_MASK as usize + 1];
        let mut remaining = target.len() as u64;
        let max = target.len() as u64;

        // Ensure the address doesn't overflow.
        address.checked_add(target.len() as u64).ok_or(io::Error::new(ErrorKind::UnexpectedEof, "Buffer with stream position overflows"))?;

        loop {
            // Read length
            let start = Self::extract_item(address);
            let end = remaining.min(Self::PAGE_ITEM_MASK + 1);
            let read_length = end.checked_sub(start).unwrap();

            let translated = self.translate_address(address).expect("Could not translate address");
            self.memory.seek(SeekFrom::Start(translated))?;

            // Read only the length needed
            let mut capped_reader = self.memory.take(read_length);
            let received = capped_reader.read(&mut frame)? as u64;
            if received == 0 { break; }
            remaining -= received;

            // Write back to output buffer
            let received_data = &frame[0..read_length as usize];
            target[bytes_read..read_length as usize + bytes_read].copy_from_slice(received_data);

            if remaining == 0 { break; }

            // Prepare next frame
            bytes_read = (max - remaining) as usize;
            address = Self::page_to_layer(Self::extract_page(address) + 1);
        }

        Ok(target.len() - remaining as usize)
    }
}

// FIXME: Rust compiler error forces "+ 'a". issue opened by x4exr on github. Should be removed when resolved.
impl<'a, Memory: Seek + Read + 'a> Read for Paged<'a, Memory> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.operate(buf, |context| {
            todo!()
        })
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