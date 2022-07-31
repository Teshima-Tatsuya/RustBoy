use crate::types::*;

pub fn bytes_2_word(upper: Byte, lower: Byte) -> Word {
    return (upper as Word) << 8 | (lower as Word);
}

pub fn extract_lower(addr: Word) -> Byte {
    return (addr & 0x00ff) as Byte;
}

pub fn extract_upper(addr: Word) -> Byte {
    return (addr >> 8) as Byte;
}

pub fn bit(v: &Byte, i: &u8) -> Byte {
    (v >> i) & 0x01
}

pub fn set_bit(v: &Byte, i: &u8) -> Byte {
    v | (1 << i)
}