//! Instruction coding format is as follows in the same order (separated into clusters of a single byte)
//! - Classification
//! - Destination
//! ---
//! - Operation
//! - Addressing Method
//! - Addressing Mode
//! ---
//! - Exponent
//! - First Register
//! - Second Register
//! ---
//! - Immediate 0-8 bytes

pub mod decoder;
pub mod encoder;