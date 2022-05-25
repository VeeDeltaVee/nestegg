pub fn is_negative(byte: u8) -> bool {
    (byte & (1 << 7)) != 0
}
