use proc_bitfield::bitfield;

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