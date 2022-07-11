use crate::types::*;

pub fn bytes_2_word(lower: Byte, upper: Byte) -> Word {
    return (upper as Word) << 8 | (lower as Word);
}

pub fn extract_lower(addr: Word) -> Byte {
    return (addr & 0x00ff) as Byte;
}

pub fn extract_upper(addr: Word) -> Byte {
    return (addr >> 8) as Byte;
}
