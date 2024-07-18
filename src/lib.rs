#![feature(seek_stream_len)]
#![feature(let_chains)]
#![deny(clippy::trivially_copy_pass_by_ref)]
#![deny(clippy::large_types_passed_by_value)]
#![allow(clippy::unusual_byte_groupings)]
#![allow(const_evaluatable_unchecked)]
#![allow(clippy::unused_io_amount)]

pub mod dynamic_number;
pub mod instruction;
pub mod memory;
pub mod core;