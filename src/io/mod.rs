mod serial;
mod apu;

use crate::{
    constant::*,
    types::*,
    traits::*,
    memory::*,
};

pub struct Io {
    serial: serial::Serial,
    apu: apu::Apu,
    tmp_buf: RAM,
}

impl Io {
    pub fn new() -> Self {
        Io {
            serial: serial::Serial::new(),
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
            ADDR_APU_NR10..=ADDR_APU_NR52 => self.apu.write(addr, value),
            v => unreachable!("Cannot write addr {:04X} for Io",v)
        }

    }
}