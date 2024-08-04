// Suffer the consequences when they aren't watching.
#![allow(incomplete_features)]

#![feature(seek_stream_len)]
#![feature(test)]
#![feature(let_chains)]
#![feature(effects)]
#![feature(const_trait_impl)]
#![deny(clippy::trivially_copy_pass_by_ref)]
#![deny(clippy::large_types_passed_by_value)]
#![allow(clippy::unusual_byte_groupings)]
#![deny(clippy::missing_const_for_fn)]
#![allow(const_evaluatable_unchecked)]
#![allow(clippy::unused_io_amount)]
#![allow(soft_unstable)]

use std::fmt::Debug;
use std::io::Cursor;

// pub mod core;
pub mod math;
pub mod paged;
pub mod instruction;
mod text;

/// Testing utility to streamline the processing of creating a temporary cursor and using it for an operation that 
/// returns a value.
/// 
/// # Result
/// Value returned by the handle closure.
pub fn read_cursor<T: AsRef<[u8]>, R>(data: T, mut handle: impl FnMut(&mut Cursor<T>) -> R) -> R {
    let mut cursor = Cursor::new(data);
    handle(&mut cursor)
}

/// Testing utility to streamline the processing of creating a temporary cursor and using it for an operation that 
/// modifies a buffer.
///
/// # Result
/// The buffer that was modified.
pub fn write_cursor<T: AsRef<[u8]>, R: Debug, X>(data: T, mut handle: impl FnMut(&mut Cursor<T>) -> Result<X, R>) -> T {
    let mut cursor = Cursor::new(data);
    handle(&mut cursor).expect("Failed write due to process error");
    cursor.into_inner()
}