pub fn combine(high_nibble: u8, low_nibble: u8) -> u8 {
    (high_nibble & 0xF0) | (low_nibble & 0x0F)
}

pub fn high_nibble_equals(source: u8, high_nibble: u8) -> bool {
    source & 0xF0 == high_nibble
}

pub fn get_tiny_size(source: u8) -> usize {
    (source & 0x0F) as usize
}