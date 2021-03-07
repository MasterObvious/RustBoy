pub fn bytes_to_word(high_byte: u8, low_byte: u8) -> u16 {
    ((high_byte as u16) << 8) | low_byte as u16
}
