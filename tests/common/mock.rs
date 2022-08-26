use rust_boy::traits::*;
use rust_boy::types::*;
use rust_boy::io::interrupt::Interrupt;
use rust_boy::io::timer::Timer;

pub struct MockBus {
    buf: [Byte; 0xFFFF],
    interrupt: Interrupt,
    timer: Timer,
}

impl MockBus {
    pub fn new() -> Self {
        Self { buf: [0; 0xFFFF], interrupt: Interrupt::new(), timer: Timer::default() }
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

impl BusAccessor for MockBus {
    fn interrupt(&mut self) -> &mut Interrupt {
        &mut self.interrupt
    }
}