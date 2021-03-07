pub fn bytes_to_word(high_byte: u8, low_byte: u8) -> u16 {
    ((high_byte as u16) << 8) | low_byte as u16
}

pub fn word_to_bytes(word: u16) -> (u8, u8) {
    let low_byte = word as u8;
    let high_byte = (word >> 8) as u8;

    (high_byte, low_byte)
}
