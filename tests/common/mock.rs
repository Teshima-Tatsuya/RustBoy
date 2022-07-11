use rust_boy::traits::*;
use rust_boy::types::*;

pub struct MockBus {
    buf: [Byte; 0xFFFF],
}

impl MockBus {
    pub fn new() -> Self {
        Self { buf: [0; 0xFFFF] }
    }
}
impl Reader for MockBus {
    fn read(&self, addr: Word) -> Byte {
        self.buf[addr as usize]
    }
}

impl Writer for MockBus {
    fn write(&mut self, addr: Word, value: Byte) {
        self.buf[addr as usize] = value
    }
}
