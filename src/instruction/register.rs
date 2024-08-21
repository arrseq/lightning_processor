use proc_bitfield::{bitfield, ConvRaw};

#[derive(Debug, Clone, Copy, PartialEq, Default, ConvRaw)]
#[repr(u8)]
pub enum File {
    #[default]
    General,
    Vector,
    Interrupt,
    Processor
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct BuildVectorOperation(pub u32): Debug, FromRaw, IntoRaw { 
        pub source_0: u8 @ 7..=11,
        pub source_1: u8 @ 12..=16,
        pub source_2: u8 @ 17..=21,
        pub source_3: u8 @ 22..=26,
        pub destination: u8 @ 27..=31
    }
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct UnBuildVectorOperation(pub u32): Debug, FromRaw, IntoRaw { 
        pub destination_0: u8 @ 7..=11,
        pub destination_1: u8 @ 12..=16,
        pub destination_2: u8 @ 17..=21,
        pub destination_3: u8 @ 22..=26,
        pub source: u8 @ 27..=31
    }
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct CopyOperation(pub u32): Debug, FromRaw, IntoRaw { 
        pub source_file: u8 [unsafe! File] @ 5..=6,
        pub destination_file: u8 [unsafe! File] @ 7..=8,
        pub source: u8 @ 22..=26,
        pub destination: u8 @ 27..=31
    }
}