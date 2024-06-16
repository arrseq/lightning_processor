use std::collections::HashMap;
use std::iter::Map;
use crate::number;

// region: Constants
pub const DUAL_ALIGNED_MASK   : u64 = 0b1;
pub const WORD_ALIGNED_MASK   : u64 = 0b11;
pub const QUAD_ALIGNED_MASK   : u64 = 0b111;
pub const PAGE_ITEM_BITS      : u64 = 13;
pub const PAGE_IDENTIFIER_MASK: u64 = u64::MAX << PAGE_ITEM_BITS;
pub const PAGE_ITEM_MASK      : u64 = u64::MAX >> (64 - PAGE_ITEM_BITS);
// endregion

/// An address frame which includes a memory address and the frame size.
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
    pub bytes: Vec<u64>,
    pub max_address: Option<u64>,
    /// Number of bytes in each page.
    pub page_size: u64,
    /// Map containing page table mappings with a process association. They first key contains the process identifier
    /// and the second contains the page table entry.
    pub pages: HashMap<u64, u64>
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
    OutOfBounds
}

impl Memory {
    pub fn read_frame() {

    }

    /// Read and return the data targeted by the frame with safeguards and emulated hardware limitations.
    /// ```
    /// use atln_processor::memory::Memory;
    ///
    /// let memory = Memory::from(Vec::from([ u64::MAX << 8 ]));
    /// // assert_eq!(memory.at(0, number::Type::Byte).unwrap().quad(), u8::MAX as u64);
    /// ```
    pub fn read(&self, frame: &Frame) -> Result<number::Data, ReadError> {
        // TODO: Implement virtual memory

        // Make sure the frame bounds lies in the memory size range.
        if let Some(max_address) = self.max_address && frame.max_address() > max_address
            { return Err(ReadError::OutOfBounds) }
        // Ensure the frame is aligned to emulate hardware limitations.
        if !frame.is_aligned() { return Err(ReadError::UnalignedFrame) }

        Ok(match frame.size {
            number::Type::Byte => number::Data::Byte(0),
            number::Type::Word => number::Data::Byte(0),
            number::Type::Dual => number::Data::Byte(0),
            number::Type::Quad => number::Data::Byte(0)
        })
    }
}

impl From<Vec<u64>> for Memory {
    /// Initialize the memory from a vector. The length of the vector is used to set the max address of the memory.
    fn from(value: Vec<u64>) -> Self {
        Self {
            max_address: Some(value.len() as u64),
            page_size: 0,
            bytes: value,
            pages: HashMap::new(),
        }
    }
}