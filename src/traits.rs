use crate::types::*;

pub trait Reader {
    fn read(&self, addr: Word) -> Byte;
}

pub trait Writer {
    fn write(&mut self, addr: Word, value: Byte);
}
