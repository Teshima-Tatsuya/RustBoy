use crate::types::*;

pub fn Bytes2Word(lower: Byte, upper: Byte) -> Word {
    return (upper as Word) << 8 | (lower as Word);
}
