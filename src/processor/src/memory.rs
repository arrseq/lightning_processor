use std::collections::HashMap;
use std::iter::Map;
use crate::number;
use crate::number::{BYTE_SIZE, DUAL_SIZE, QUAD_SIZE, WORD_SIZE};

// region: Constants
pub const DUAL_ALIGNED_MASK   : u64 = 0b1;
pub const WORD_ALIGNED_MASK   : u64 = 0b11;
pub const QUAD_ALIGNED_MASK   : u64 = 0b111;
pub const PAGE_ITEM_BITS      : u64 = 13;
pub const PAGE_IDENTIFIER_MASK: u64 = u64::MAX << PAGE_ITEM_BITS;
pub const PAGE_ITEM_MASK      : u64 = u64::MAX >> (64 - PAGE_ITEM_BITS);
pub const PAGES_MAX           : u64 = u64::MAX & PAGE_IDENTIFIER_MASK;
// endregion

/// An address frame which includes a memory address and the frame size.
#[derive(Debug, Clone)]
pub struct Frame {
    pub address: u64,
    pub size: number::Type
}

impl Frame {
    /// Check to see if the current address frame is aligned to memory. Only aligned frames can be used to interact
    /// with memory.
    /// ```
    /// use atln_processor::memory::Frame;
    /// use atln_processor::number::Type;
    ///
    /// // Aligned
    /// assert!(Frame { address: 0, size: Type::Byte }.is_aligned());
    /// assert!(Frame { address: 0, size: Type::Quad }.is_aligned());
    /// assert!(Frame { address: 7, size: Type::Byte }.is_aligned());
    ///
    /// assert!(Frame { address: 8, size: Type::Word }.is_aligned());
    /// assert!(Frame { address: 8, size: Type::Quad }.is_aligned());
    ///
    /// // Not aligned
    /// assert!(!Frame { address: 7, size: Type::Word }.is_aligned());
    /// assert!(!Frame { address: 1, size: Type::Quad }.is_aligned());
    /// ```
    pub fn is_aligned(&self) -> bool {
        let masked = match self.size {
            number::Type::Byte => 0,
            number::Type::Word => self.address & WORD_ALIGNED_MASK,
            number::Type::Dual => self.address & DUAL_ALIGNED_MASK,
            number::Type::Quad => self.address & QUAD_ALIGNED_MASK
        };

        masked == 0
    }

    /// Gets the largest targeted address.
    pub fn max_address(&self) -> u64 {
        self.address + self.size.size() as u64
    }
}

/// A page remapping utility.
pub trait Page {
    fn with_virtual(&self, r#virtual: u64) -> u64;
}

// region: TODO: Rust as of now does not implement Read for Vec<u8>. These functions should be replaced when rust
//         TODO: supports them.

/// Read a vector like a stream. Read buffer.len() amount of bytes from the vector and into the buffer. This will return
/// the number of bytes read.
fn read_vec(vec: &Vec<u8>, start: usize, buffer: &mut [u8]) -> usize {
    let mut bytes_read = 0;
    for index in 0..buffer.len() {
        match vec.get(start + index) {
            Some(value) => buffer[start + index] = *value,
            None => {
                bytes_read += 1;
                return bytes_read;
            }
        }
    }
    
    bytes_read
}
// endregion

impl Page for u64 {
    /// Translate a virtual address into a physical address.
    /// ```
    /// use atln_processor::memory::Page;
    /// 
    /// // TODO: Exhaustive testing potentially required.
    /// assert_eq!(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000.with_virtual(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111), 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111);
    /// assert_eq!(0b00000000_00000000_00000000_00000000_00000000_00000000_00100000_00000000.with_virtual(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011), 0b00000000_00000000_00000000_00000000_00000000_00000000_00100000_00000011);
    /// assert_eq!(0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000.with_virtual(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001010), 0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00001010);
    /// ```
    fn with_virtual(&self, r#virtual: u64) -> u64 {
        let page_item = r#virtual & PAGE_ITEM_MASK;
        (self & PAGE_IDENTIFIER_MASK) | page_item
    }
}

/// Memory addressing must be aligned. Rules must be followed for frame based operations on memory.
/// - If the memory is size constrained, then ensure the frame is not reaching past the memory size limit.
/// - Frames must be aligned to simulate hardware limitations of an implemented memory module.
#[derive(Debug, Clone)]
pub struct Memory {
    pub bytes: Vec<u8>,
    pub max_address: Option<u64>,
    /// Number of bytes in each page.
    pub page_size: u64,
    /// Map containing page table mappings with a context association. They first key contains the context identifier
    /// and the second contains the page table entry.
    ///
    /// The page table entry contains two parts, the virtual address prefix, and the page to remap to.
    pub pages: HashMap<u64, HashMap<u64, u64>>,
    /// The current context code. If [None] is set, then there is no context, otherwise if [Some] is used then virtual
    /// memory should be used and use/create pages associated with the context identifier.
    ///
    /// The determined whether virtual memory mapping happens.
    pub context: Option<u64>
}

/// Error caused from setting data in memory.
pub enum SetError {
    /// Error from using an unaligned address frame.
    UnalignedFrame
}
 
/// Caused by invalid parameters to initialize an address frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadError {
    /// The memory address requested data that is sized outside the memory aligned divisions.
    UnalignedFrame,
    /// The address frame crosses the positive memory boundaries.
    OutOfBounds,
    /// Virtual memory context was in use but the remapping did not exist in tge page list.
    PageFault
}

impl Memory {
    /// Get a specific page based on the context.
    pub fn get_pages(&self, context: u64) -> Option<&HashMap<u64, u64>> {
        self.pages.get(&context)
    }

    /// Translate the virtual address into a physical address based on the current situation. This returns a unit if the
    /// page mapping does not exist. This is a page fault.
    /// - The context is a unique code that allows pages to be groups. This could be used for organizing processes or
    ///   other execution context's, hence the name, a context.
    /// - The virtual address is the address that what ever is executing in a context see's.
    ///
    /// We are parameterizing the context because the context could potentially be [None]. This is to execute only with
    /// a valid context.
    /// ```
    /// use std::collections::HashMap;
    /// use atln_processor::memory::Memory;
    ///
    /// let mut memory = Memory::from(Vec::new());
    /// memory.pages = HashMap::from([
    ///     (1, HashMap::from([ (10, 200) ])),
    ///     (0, HashMap::from([ (10, 200) ])),
    /// 
    ///     // Pages that are next to each other.
    ///     (100, HashMap::from([ 
    ///         (10, 200),
    ///         (9, 199),
    ///         (8, 198)
    ///     ]))
    /// ]);
    ///
    /// // Test different contexts.
    /// assert_eq!(memory.translate_virtual(1, 0b000_00000000_00000000_00000000_00000000_00000000_00001010__00000_00000011).unwrap(), 0b000_00000000_00000000_00000000_00000000_00000000_11001000__00000_00000011);
    /// assert_eq!(memory.translate_virtual(0, 0b000_00000000_00000000_00000000_00000000_00000000_00001010__00000_00000011).unwrap(), 0b000_00000000_00000000_00000000_00000000_00000000_11001000__00000_00000011);
    /// 
    /// // Test multiple mappings.
    /// assert_eq!(memory.translate_virtual(100, 0b000_00000000_00000000_00000000_00000000_00000000_00001010__00000_00001010).unwrap(), 0b000_00000000_00000000_00000000_00000000_00000000_11001000__00000_00001010);
    /// assert_eq!(memory.translate_virtual(100, 0b000_00000000_00000000_00000000_00000000_00000000_00001001__00000_00001010).unwrap(), 0b000_00000000_00000000_00000000_00000000_00000000_11000111__00000_00001010);
    /// assert_eq!(memory.translate_virtual(100, 0b000_00000000_00000000_00000000_00000000_00000000_00001000__00000_00001010).unwrap(), 0b000_00000000_00000000_00000000_00000000_00000000_11000110__00000_00001010);
    /// 
    /// // region: Page fault.
    /// // Invalid context
    /// assert!(matches!(memory.translate_virtual(199, 0b000_00000000_00000000_00000000_00000000_00000000_00001000__00000_00001010), None));
    /// // Unmapped page.
    /// assert!(matches!(memory.translate_virtual(200, 0b000_00000000_00000000_00000000_00000000_00000000_00000000__00000_00001010), None));
    /// // endregion
    /// ```
    pub fn translate_virtual(&self, context: u64, r#virtual: u64) -> Option<u64> {
        // This is the page identifier of the virtual address. The virtual address space doesn't have pages on its
        // own but this is used to find out what the page mapping is. Thus making it the prefix.
        //
        // Shift the bits right to allow for it to be treated as a real number.
        let virtual_prefix = (PAGE_IDENTIFIER_MASK & r#virtual) >> PAGE_ITEM_BITS;

        // If no page table group exists for the context then this will also result in a page fault.
        let pages = match self.get_pages(context) {
            Some(pages) => pages,
            None => return None
        };

        // Try to get the page mapping item itself. No match will cause a page fault. For safety, ensure that the page
        // mapping identifier does not use more bits than is supported.
        let physical_prefix = match pages.get(&virtual_prefix) {
            // it's ok to dereference a number.
            Some(value) => *value,
            None => return None
        }
            // Used as a mask, needs to be shifted over to allow for it to layer on an item suffix. This also behaves
            // as removing the items bits.
            << PAGE_ITEM_BITS;

        // Remove the prefix bits. This will make sure nothing goes wrong when doing the "or" operation.
        let virtual_suffix = PAGE_ITEM_MASK & r#virtual;

        // use the virtual address suffix to select the individual byte and the physical prefix to select the page
        // block.
        Some(virtual_suffix | physical_prefix)
    }

    /// Read and return the data targeted by the frame with safeguards and emulated hardware limitations. If the page
    /// is not cached in this list, then a [ReadError::PageFault] is caused.
    /// ```
    /// use atln_processor::memory::Memory;
    ///
    /// let memory = Memory::from(Vec::from([ u64::MAX << 8 ]));
    /// // assert_eq!(memory.at(0, number::Type::Byte).unwrap().quad(), u8::MAX as u64);
    /// ```
    pub fn read(&self, frame: &Frame) -> Result<number::Data, ReadError> {
        let mut address_start = frame.address;
        if let Some(context) = self.context {
            address_start = match self.translate_virtual(context, frame.address) {
                Some(value) => value,
                None => return Err(ReadError::PageFault)
            };
        }
        
        // New frame with potential for translated address.
        let mut frame = frame.clone();
        frame.address = address_start;
        
        // Make sure the frame bounds lies in the memory size range.
        if let Some(max_address) = self.max_address && frame.max_address() > max_address
            { return Err(ReadError::OutOfBounds) }
        // Ensure the frame is aligned to emulate hardware limitations.
        if !frame.is_aligned() { return Err(ReadError::UnalignedFrame) }

        let mut max_buffer = [0u8; QUAD_SIZE as usize];
        Ok(match frame.size {
            number::Type::Byte => {
                let buffer = &mut max_buffer[0..BYTE_SIZE as usize];
                if read_vec(&self.bytes, address_start as usize, buffer) != buffer.len() { return Err(ReadError::OutOfBounds) }
                number::Data::Byte(buffer[0])
            },
            number::Type::Word => {
                let buffer = &mut max_buffer[0..WORD_SIZE as usize];
                if read_vec(&self.bytes, address_start as usize, buffer) != buffer.len() { return Err(ReadError::OutOfBounds) }
                number::Data::Word(u16::from_le_bytes([ buffer[0], buffer[1] ]))
            },
            number::Type::Dual => {
                let buffer = &mut max_buffer[0..DUAL_SIZE as usize];
                if read_vec(&self.bytes, address_start as usize, buffer) != buffer.len() { return Err(ReadError::OutOfBounds) }
                number::Data::Dual(u32::from_le_bytes([ buffer[0], buffer[1], buffer[2], buffer[3] ]))
            },
            number::Type::Quad => {
                let buffer = &mut max_buffer[0..QUAD_SIZE as usize];
                if read_vec(&self.bytes, address_start as usize, buffer) != buffer.len() { return Err(ReadError::OutOfBounds) }
                number::Data::Quad(u64::from_le_bytes([ buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5], buffer[6], buffer[7] ]))
            }
        })
    }
}

impl From<Vec<u8>> for Memory {
    /// Initialize the memory from a vector. The length of the vector is used to set the max address of the memory.
    fn from(value: Vec<u8>) -> Self {
        Self {
            max_address: Some(value.len() as u64),
            page_size: 0,
            bytes: value,
            pages: HashMap::new(),
            context: None
        }
    }
}