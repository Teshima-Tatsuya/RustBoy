mod serial;
pub mod timer;
pub mod interrupt;
mod apu;

use std::{
    cell::RefCell,
    rc::Rc,
};

use crate::{
    constant::*,
    types::*,
    traits::*,
    memory::*,
};

pub struct Io {
    serial: serial::Serial,
    pub timer: Rc<RefCell<timer::Timer>>,
    pub interrupt: interrupt::Interrupt,
    apu: apu::Apu,
    tmp_buf: RAM,
}

impl Io {
    pub fn new(timer: Rc<RefCell<timer::Timer>>) -> Self {
        Io {
            serial: serial::Serial::new(),
            timer: timer,
            interrupt: interrupt::Interrupt::new(),
            apu: apu::Apu::new(),
            tmp_buf: RAM::new(0x10000),
        }
    }
}

impl Reader for Io {
    fn read(&self, addr: Word) -> Byte {
        match addr {
            ADDR_JOYPAD => self.tmp_buf.read(addr),
            ADDR_SERIAL_SB..=ADDR_SERIAL_SC => self.serial.read(addr),
            ADDR_TIMER_DIV..=ADDR_TIMER_TAC => self.timer.borrow().read(addr),
            ADDR_INTERRUPT_IF | ADDR_INTERRUPT_IE => self.interrupt.read(addr),
            ADDR_APU_NR10..=ADDR_APU_NR52 => self.apu.read(addr),
            v => unreachable!("Cannot read addr {:04X} for Io",v)
        }
    }
}

impl Writer for Io {
    fn write(&mut self, addr: Word, value: Byte) {
        match addr {
            ADDR_JOYPAD => self.tmp_buf.write(addr, value),
            ADDR_SERIAL_SB..=ADDR_SERIAL_SC => self.serial.write(addr, value),
            ADDR_TIMER_DIV..=ADDR_TIMER_TAC => self.timer.borrow_mut().write(addr, value),
            ADDR_INTERRUPT_IF | ADDR_INTERRUPT_IE => self.interrupt.write(addr, value),
            ADDR_APU_NR10..=ADDR_APU_NR52 => self.apu.write(addr, value),
            v => unreachable!("Cannot write addr {:04X} for Io",v)
        }

    }
}