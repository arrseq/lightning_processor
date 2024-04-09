pub struct DynNum {
    u_bytes: Vec<u8>
}

impl DynNum {
    pub fn from_uint(uint: usize) -> Self {
        // Convert the usize to bytes using little-endian
        let bytes = uint.to_le_bytes();

        // If the number is zero, add a single byte of zero
        let mut u_bytes = Vec::new();
        if uint == 0 {
            u_bytes.push(0);
        } else {
            u_bytes.extend_from_slice(&bytes);
        }

        Self {
            u_bytes
        }
    }
}