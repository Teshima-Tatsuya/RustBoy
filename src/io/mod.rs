mod serial;
mod timer;
mod interrupt;

use crate::{
    constant::*,
    types::*,
    traits::*,
    memory::*,
};

pub struct Io {
    serial: serial::Serial,
    timer: timer::Timer,
    interrupt: interrupt::Interrupt,
    tmp_buf: RAM,
}

impl Io {
    pub fn new() -> Self {
        Io {
            serial: serial::Serial::new(),
            timer: timer::Timer::new(),
            interrupt: interrupt::Interrupt::new(),
            tmp_buf: RAM::new(0x10000),
        }
    }
}

impl Reader for Io {
    fn read(&self, addr: Word) -> Byte {
        match addr {
            ADDR_JOYPAD => self.tmp_buf.read(addr),
            ADDR_SERIAL_SB..=ADDR_SERIAL_SC => self.serial.read(addr),
            ADDR_TIMER_DIV..=ADDR_TIMER_TAC => self.timer.read(addr),
            v => unreachable!("Cannot read addr {:04X} for Io",v)
        }
    }
}

impl Writer for Io {
    fn write(&mut self, addr: Word, value: Byte) {
        match addr {
            ADDR_JOYPAD => self.tmp_buf.write(addr, value),
            ADDR_SERIAL_SB..=ADDR_SERIAL_SC => self.serial.write(addr, value),
            ADDR_TIMER_DIV..=ADDR_TIMER_TAC => self.timer.write(addr, value),
            v => unreachable!("Cannot write addr {:04X} for Io",v)
        }

    }
}