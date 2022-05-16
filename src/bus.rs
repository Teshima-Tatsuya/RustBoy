use crate::rom::Rom;
use crate::traits::*;
use crate::types::*;

struct Bus {
    rom: Rom,
}

impl ReaderWriter for Bus {
    fn read(addr: Word) -> Byte {
        0
    }
    fn write(addr: Word, value: Byte) {}
}
