use crate::types::*;

pub trait ReaderWriter {
    fn read(addr: Word) -> Byte;
    fn write(addr: Word);
}
