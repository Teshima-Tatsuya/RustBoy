use crate::types::*;

pub fn Bytes2Word(lower: Byte, upper: Byte) -> Word {
    return (upper as Word) << 8 | (lower as Word);
}

pub fn ExtractLower(addr: Word) -> Byte {
    return (addr & 0x00ff) as Byte;
}

pub fn ExtractUpper(addr: Word) -> Byte {
    return (addr >> 8) as Byte;
}
