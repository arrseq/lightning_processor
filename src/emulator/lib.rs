//! # Declarations
//! - References to parts of a binary number will be in big endian. Saying left most bits would refer to the most 
//!   significant bits.
//! - The architecture this library implements is for the xT3 processor. xT3 also reefers to the architecture of the
//!   processor.
//! 
//! # Terms
//! - Real mode is a state of the processor that allows addressing to be as is rather than being translated. An 
//!   operating system kernel would run in real mode.

#![forbid(clippy::result_unit_err)]
#![forbid(clippy::question_mark)]
#![allow(clippy::module_inception)]

pub mod graphics;
pub mod memory;
pub mod number;
pub mod processor;
pub mod utility;