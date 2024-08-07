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

pub mod instruction;
pub mod num;
pub mod operation;
pub mod state;