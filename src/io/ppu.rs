use crate::{
    types::*,
    traits::*,
    constant::*,
    memory::*,
};

#[derive(Default)]
pub struct Ppu {
    buf: RAM,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            buf: RAM::new(0xFFFF)
        }
    }
}

impl Reader for Ppu {
    fn read(&self, addr: Word) -> Byte {
        self.buf.read(addr)
    }
}

impl Writer for Ppu {
    fn write(&mut self, addr: Word, value: Byte) {
        self.buf.write(addr, value);
    }
}