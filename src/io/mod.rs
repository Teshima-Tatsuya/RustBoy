mod serial;
mod timer;
pub mod interrupt;
mod apu;
mod ppu;

use crate::{
    constant::*,
    types::*,
    traits::*,
    memory::*,
};

pub struct Io {
    serial: serial::Serial,
    timer: timer::Timer,
    pub interrupt: interrupt::Interrupt,
    apu: apu::Apu,
    ppu: ppu::Ppu,
    tmp_buf: RAM,
}

impl Io {
    pub fn new() -> Self {
        Io {
            serial: serial::Serial::new(),
            timer: timer::Timer::default(),
            interrupt: interrupt::Interrupt::new(),
            apu: apu::Apu::new(),
            ppu: ppu::Ppu::new(),
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
            ADDR_INTERRUPT_IF | ADDR_INTERRUPT_IE => self.interrupt.read(addr),
            ADDR_APU_NR10..=ADDR_APU_NR52 => self.apu.read(addr),
            ADDR_PPU_LCDC..=ADDR_PPU_OCPD => self.ppu.read(addr),
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
            ADDR_INTERRUPT_IF | ADDR_INTERRUPT_IE => self.interrupt.write(addr, value),
            ADDR_APU_NR10..=ADDR_APU_NR52 => self.apu.write(addr, value),
            ADDR_PPU_LCDC..=ADDR_PPU_OCPD => self.ppu.write(addr, value),
            v => unreachable!("Cannot write addr {:04X} for Io",v)
        }

    }
}