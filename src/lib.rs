#![feature(seek_stream_len)]
#![feature(test)]
#![feature(let_chains)]
#![deny(clippy::trivially_copy_pass_by_ref)]
#![deny(clippy::large_types_passed_by_value)]
#![allow(clippy::unusual_byte_groupings)]
#![deny(clippy::missing_const_for_fn)]
#![allow(const_evaluatable_unchecked)]
#![allow(clippy::unused_io_amount)]
#![allow(soft_unstable)]

use std::io::Cursor;

// pub mod core;
pub mod math;
// pub mod paged;
pub mod instruction;

/// Testing utility to streamline the processing of creating a temporary cursor and using it for an operation that 
/// returns a value.
/// 
/// # Result
/// Value returned by the handle closure.
pub fn cursor_test<T: AsRef<[u8]>, R>(data: T, mut handle: impl FnMut(&mut Cursor<T>) -> R) -> R {
    let mut cursor = Cursor::new(data);
    handle(&mut cursor)
}