use crate::{
    types::*,
    traits::*,
    constant::*,
    memory::*,
};

#[derive(Default)]
pub struct Apu {
    buf: RAM,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            buf: RAM::new(0x100)
        }
    }
}

impl Reader for Apu {
    fn read(&self, addr: Word) -> Byte {
        self.buf.read(addr)
    }
}

impl Writer for Apu {
    fn write(&mut self, addr: Word, value: Byte) {
        self.buf.write(addr, value);
    }
}