pub trait Byte {
    fn into_bits(&self) -> [bool; 8];
}

impl Byte for u8 {
    fn into_bits(&self) -> [bool; 8] {
        let mut bits = [false; 8];
        for index in 0..8 {
            let shifted = self >> index;
            let current = shifted & 1;
            bits[7 - index as usize] = current == 1;
        }
        bits
    }
}

#[test]
fn test_byte_into_bits() {
    let byte = 0b10101100u8;
    let correct = [
        true, false, true, false, 
        true, true, false, false
    ];

    assert_eq!(byte.into_bits(), correct);
}

pub trait Bits {
    fn into_byte(&self) -> u8;
}

impl Bits for [bool; 8] {
    fn into_byte(&self) -> u8 {
        let mut byte = 0u8;
        for (index, &bit) in self.iter().enumerate() {
            if bit {
                byte |= 1 << (7 - index);
            }
        }
        byte
    }
} 

#[test]
fn test_bits_into_byte() {
    let bits = [
        true, false, true, false,
        true, true, false, false
    ];
    let correct = 0b10101100u8;

    assert_eq!(bits.into_byte(), correct);
}