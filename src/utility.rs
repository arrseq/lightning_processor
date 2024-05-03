pub trait Byte {
    fn into_bits(&self) -> [bool; 8];
}

impl Byte for u8 {
    fn into_bits(&self) -> [bool; 8] {
        let mut bits = [false; 8];
        for index in 0..8 {
            let shifted = self >> index;
            let current = shifted & index;
            bits[7 - index as usize] = current == 1;
        }
        bits
    }
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