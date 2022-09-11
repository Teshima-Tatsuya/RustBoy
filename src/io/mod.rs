mod serial;
mod apu;
mod joypad;

use crate::{
    constant::*,
    types::*,
    traits::*,
    memory::*,
};

pub struct Io {
    serial: serial::Serial,
    apu: apu::Apu,
    joypad: joypad::Joypad,
    tmp_buf: RAM,
}

impl Io {
    pub fn new() -> Self {
        Io {
            serial: serial::Serial::new(),
            apu: apu::Apu::new(),
            joypad: joypad::Joypad::new(),
            tmp_buf: RAM::new(0x10000),
        }
    }
}

impl Reader for Io {
    fn read(&self, addr: Word) -> Byte {
        match addr {
            ADDR_JOYPAD => self.joypad.read(addr),
            ADDR_SERIAL_SB..=ADDR_SERIAL_SC => self.serial.read(addr),
            ADDR_APU_NR10..=ADDR_APU_NR52 => self.apu.read(addr),
            v => {
                // log::warn!("Cannot read addr {:04X} for Io",v);
                0xFF
            }
        }
    }
}

impl Writer for Io {
    fn write(&mut self, addr: Word, value: Byte) {
        match addr {
            ADDR_JOYPAD => self.joypad.write(addr, value),
            ADDR_SERIAL_SB..=ADDR_SERIAL_SC => self.serial.write(addr, value),
            ADDR_APU_NR10..=ADDR_APU_NR52 => self.apu.write(addr, value),
            v => () //log::warn!("Cannot write addr {:04X} for Io",v)
        }

    }
}