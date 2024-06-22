use std::collections::HashMap;
use std::io;
use std::io::{ErrorKind, Read, Seek, SeekFrom};
use crate::number;
use crate::number::{BYTE_SIZE, DUAL_SIZE, QUAD_SIZE, Size, WORD_SIZE};

// region: Constants
pub const DUAL_ALIGNED_MASK   : u64 = 0b1;
pub const WORD_ALIGNED_MASK   : u64 = 0b11;
pub const QUAD_ALIGNED_MASK   : u64 = 0b111;

pub const PAGE_ITEM_BITS      : u64 = 13;
pub const PAGE_IDENTIFIER_MASK: u64 = u64::MAX << PAGE_ITEM_BITS;
pub const PAGE_ITEM_MASK      : u64 = u64::MAX >> (64 - PAGE_ITEM_BITS);
pub const MAX_PAGES_COUNT     : u64 = u64::MAX & PAGE_IDENTIFIER_MASK;
pub const PAGE_BYTES_COUNT    : u64 = (u64::MAX & PAGE_ITEM_MASK) + 1;

// pub const PAGE_BYTES_COUNT    : u64 = 2u64.pow(PAGE_ITEM_BITS as u32);
// endregion

// region: Binary buffer
/// Read all of a structure into another buffer of some sort. This is similar to [Read] with the difference being that
/// all data is read into the buffer and any that don't fit are simply truncated.
///
/// Use this on things such as enums or things without structures. This is jank and not good, this trait is a retro fit
/// due to poor early planing, things like [Data] are too deeply nested and implemented to be refactored into a
/// structure to then be later used with Read.
pub trait ReadAll<T> where
    T: ?Sized {
    /// Read some container and store the result inside a target somehow. This returns the number of bytes stored.
    fn read_all(&mut self, target: &mut T) -> usize;
}

pub trait LastError<E> {
    /// Get the last emitted error from a member of the parent object.
    fn last_error(&mut self) -> &Option<E>;
}
// endregion

/// An address frame which includes a memory address and the frame size.
#[derive(Debug, Clone)]
pub struct Frame {
    pub address: u64,
    pub size: number::Size
}

impl Frame {
    /// Check to see if the current address frame is aligned to memory. Only aligned frames can be used to interact
    /// with memory.
    /// ```
    /// use atln_processor::memory::Frame;
    /// use atln_processor::number::Size;
    ///
    /// // Aligned
    /// assert!(Frame { address: 0, size: Size::Byte }.is_aligned());
    /// assert!(Frame { address: 0, size: Size::Quad }.is_aligned());
    /// assert!(Frame { address: 7, size: Size::Byte }.is_aligned());
    ///
    /// assert!(Frame { address: 8, size: Size::Word }.is_aligned());
    /// assert!(Frame { address: 8, size: Size::Quad }.is_aligned());
    ///
    /// // Not aligned
    /// assert!(!Frame { address: 7, size: Size::Word }.is_aligned());
    /// assert!(!Frame { address: 1, size: Size::Quad }.is_aligned());
    /// ```
    pub fn is_aligned(&self) -> bool {
        let masked = match self.size {
            number::Size::Byte => 0,
            number::Size::Word => self.address & WORD_ALIGNED_MASK,
            number::Size::Dual => self.address & DUAL_ALIGNED_MASK,
            number::Size::Quad => self.address & QUAD_ALIGNED_MASK
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

/// Read a vector like a stream. Read buffer.len() amount of bytes from the vector and into the buffer. This will return
/// the number of bytes read.
/// ```
/// // TODO: Test
/// ```
pub fn read_vec_into_buffer(vec: &Vec<u8>, start: usize, buffer: &mut [u8]) -> usize {
    let mut bytes_read = 0;
    for index in 0..buffer.len() {
        match vec.get(start + index) {
            Some(value) => buffer[index] = *value,
            None => return bytes_read
        }
        
        bytes_read += 1;
    }
    
    bytes_read
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
#[derive(Debug, Clone, Default)]
pub struct Memory {
    pub bytes: Vec<u8>,
    pub max_address: Option<u64>,
    /// Number of bytes in each page.
    pub page_size: u64,
    /// Mappings of virtual page addresses to physical page addresses.
    pub pages: HashMap<u64, u64>,
    /// The location to start reading from. This does not apply when doing direct reads.
    pub read_head: u64,
    /// The last error caused by memory when getting data.
    pub get_error: Option<GetError>
}

/// Error caused from setting data in memory.
pub enum SetError {
    /// Error from using an unaligned address frame.
    UnalignedFrame
}

/// Caused by invalid parameters to initialize an address frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetError {
    /// The memory address requested data that is sized outside the memory aligned divisions.
    UnalignedFrame,
    /// The address frame crosses the positive memory boundaries.
    OutOfBounds,
    /// Virtual memory context was in use but the remapping did not exist in the page list.
    PageFault
}

impl Memory {
    /// Translate the virtual address into a physical address based on the current situation. This returns a unit if the
    /// page mapping does not exist. This is a page fault.
    /// If the page does not exist then that case is a page fault. This function would return [None] to imply a page
    /// fault.
    /// ```
    /// use std::collections::HashMap;
    /// use atln_processor::memory::{Memory};
    ///
    /// let mut memory = Memory::from(Vec::new());
    /// memory.pages = HashMap::from([
    ///     // Pages that are next to each other.
    ///     (10, 200),
    ///     (9, 199),
    ///     (8, 198)
    /// ]);
    ///
    /// // Test multiple mappings.
    /// assert_eq!(memory.translate_virtual(0b000_00000000_00000000_00000000_00000000_00000000_00001010__00000_00001010).unwrap(), 0b000_00000000_00000000_00000000_00000000_00000000_11001000__00000_00001010);
    /// assert_eq!(memory.translate_virtual(0b000_00000000_00000000_00000000_00000000_00000000_00001001__00000_00001010).unwrap(), 0b000_00000000_00000000_00000000_00000000_00000000_11000111__00000_00001010);
    /// assert_eq!(memory.translate_virtual(0b000_00000000_00000000_00000000_00000000_00000000_00001000__00000_00001010).unwrap(), 0b000_00000000_00000000_00000000_00000000_00000000_11000110__00000_00001010);
    ///
    /// // Unmapped page. This is a page fault situation.
    /// assert!(matches!(memory.translate_virtual(0b000_00000000_00000000_00000000_00000000_00000000_00000000__00000_00001010), None));
    /// ```
    pub fn translate_virtual(&self, r#virtual: u64) -> Option<u64> {
        // This is the page identifier of the virtual address. The virtual address space doesn't have pages on its
        // own but this is used to find out what the page mapping is. Thus making it the prefix.
        //
        // Shift the bits right to allow for it to be treated as a real number.
        let virtual_prefix = (PAGE_IDENTIFIER_MASK & r#virtual) >> PAGE_ITEM_BITS;

        // Try to get the page mapping item itself. No match will cause a page fault. For safety, ensure that the page
        // mapping identifier does not use more bits than is supported.
        let physical_prefix = self.pages.get(&virtual_prefix)?
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
    /// is not cached in this list, then a [GetError::PageFault] is caused.
    /// ```
    /// use std::collections::HashMap;
    /// use atln_processor::memory::{Frame, Memory, PAGE_BYTES_COUNT, PAGE_ITEM_BITS};
    /// use atln_processor::number::{Data, Size};
    ///
    /// // region: Basic non virtual addressing.
    /// let mut memory = Memory::from(Vec::from([ 0, 0, 0, 0 ]));
    /// assert_eq!(memory.get(&Frame { address: 0, size: Size::Dual }).unwrap(), Data::Dual(0));
    ///
    /// let mut memory = Memory::from(Vec::from([ 255, 255, 255, 255, 0, 0, 0, 0 ]));
    /// assert_eq!(memory.get(&Frame { address: 0, size: Size::Quad }).unwrap(), Data::Quad(u32::MAX as u64));
    ///
    /// let mut memory = Memory::from(Vec::from(1001u64.to_le_bytes()));
    /// assert_eq!(memory.get(&Frame { address: 0, size: Size::Quad }).unwrap(), Data::Quad(1001));
    /// assert_eq!(memory.get(&Frame { address: 1, size: Size::Byte }).unwrap(), Data::Byte(3));
    /// // endregion
    /// 
    /// // region: Test virtual memory. This is very address specific and everything must work perfectly.
    /// let mut memory = Memory::from({
    ///     let mut store = vec![0u8; (PAGE_BYTES_COUNT * 2) as usize];
    ///   
    ///     // Memory addresses are zero indexed.
    ///     let second_page_index = PAGE_BYTES_COUNT as usize;
    ///
    ///     store[second_page_index] = 255;
    ///     store[second_page_index + 1] = 1;
    ///
    ///     // To account for memory alignment.
    ///     store[second_page_index + 5] = 1;
    ///     store[second_page_index + 6] = 255;
    ///     
    ///     store
    /// });
    ///
    /// // Map addresses from first virtual page boundary to the second hardware page. Hardware and virtual pages align 
    /// // parallel.
    /// memory.pages.insert(0, 1);
    ///
    /// // Test.
    /// assert_eq!(memory.get(&Frame { address: 0, size: Size::Byte }).unwrap(), Data::Byte(255));
    /// assert_eq!(memory.get(&Frame { address: 0, size: Size::Word }).unwrap(), Data::Word(511));
    /// assert_eq!(memory.get(&Frame { address: 4, size: Size::Word }).unwrap(), Data::Word(256));
    /// // endregion
    /// ```
    pub fn get(&mut self, frame: &Frame) -> Result<number::Data, GetError> {
        dbg!(frame.address);

        fn inner(memory: &Memory, frame: &Frame) -> Result<number::Data, GetError> {
            // region: Addressing
            // TODO: Make this a separate function with its own tests.
            let mut address_start = frame.address;
            // TODO: Add translation signal.
            if false {
                address_start = match memory.translate_virtual(frame.address) {
                    Some(value) => value,
                    None => return Err(GetError::PageFault)
                };
            }

            // New frame with potential for translated address.
            let mut frame = frame.clone();
            frame.address = address_start;

            // Make sure the frame bounds lies in the memory size range.
            if let Some(max_address) = memory.max_address && frame.max_address() > max_address
            { return Err(GetError::OutOfBounds) }
            // Ensure the frame is aligned to emulate hardware limitations.
            if !frame.is_aligned() { return Err(GetError::UnalignedFrame) }
            // endregion

            let mut max_buffer = [0u8; QUAD_SIZE];
            Ok(match frame.size {
                Size::Byte => {
                    let buffer = &mut max_buffer[0..BYTE_SIZE];
                    if read_vec_into_buffer(&memory.bytes, address_start as usize, buffer) != buffer.len() { return Err(GetError::OutOfBounds) }
                    number::Data::Byte(buffer[0])
                },
                Size::Word => {
                    let buffer = &mut max_buffer[0..WORD_SIZE];
                    if read_vec_into_buffer(&memory.bytes, address_start as usize, buffer) != buffer.len() { return Err(GetError::OutOfBounds) }
                    number::Data::Word(u16::from_le_bytes([ buffer[0], buffer[1] ]))
                },
                Size::Dual => {
                    let buffer = &mut max_buffer[0..DUAL_SIZE];
                    if read_vec_into_buffer(&memory.bytes, address_start as usize, buffer) != buffer.len() { return Err(GetError::OutOfBounds) }
                    number::Data::Dual(u32::from_le_bytes([ buffer[0], buffer[1], buffer[2], buffer[3] ]))
                },
                Size::Quad => {
                    let buffer = &mut max_buffer[0..QUAD_SIZE];
                    if read_vec_into_buffer(&memory.bytes, address_start as usize, buffer) != buffer.len() { return Err(GetError::OutOfBounds) }
                    number::Data::Quad(u64::from_le_bytes([ buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5], buffer[6], buffer[7] ]))
                }
            })
        }

        match inner(self, frame) {
            Ok(value) => {
                self.get_error = None;
                Ok(value)
            },
            Err(error) => {
                self.get_error = Some(error.clone());
                Err(error)
            }
        }
    }
}

impl LastError<GetError> for Memory {
    fn last_error(&mut self) -> &Option<GetError> {
        &self.get_error
    }
}

impl Read for Memory {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let size = match Size::from_size(buf.len()) {
            Some(value) => value,
            None => return Err(io::Error::new(ErrorKind::Other, "Invalid buffer length"))
        };
        
        let mut data = match self.get(&Frame { address: self.read_head, size }) {
            Ok(result) => result,
            // Memory errors can be accessed after this function by executing
            // LastError<GetError>::last_error(&mut Memory).
            Err(_) => return Err(io::Error::new(ErrorKind::Other, "Failed to read from memory"))
        };

        Ok(data.read_all(buf))
    }
}

impl Seek for Memory {
    /// ```
    /// // TODO; Test
    /// ```
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match pos {
            SeekFrom::Start(start) => self.read_head = start,
            SeekFrom::End(end) => self.read_head = (self.bytes.len() as i64 - end) as u64,
            SeekFrom::Current(curr) => self.read_head = (self.read_head as i64 + curr) as u64
        }
        
        Ok(self.read_head)
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
            read_head: 0,
            get_error: None
        }
    }
}