use proc_bitfield::bitfield;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Operation(pub u32): Debug, FromRaw, IntoRaw { 
        pub segment: u8 @ 5..=6,
        pub value: u16 @ 11..=26,
        pub base: u8 @ 27..=31
    }
}