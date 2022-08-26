use std::{
    cell::RefCell,
    rc::Rc,
};

use crate::{
    traits::*,
    types::*,
    {mbc::Mbc, mbc::MbcTrait},
    memory::*,
    io::*,
};

pub struct Bus {
    mbc: Mbc,
    vram: RAM,
    wram: RAM,
    wram2: RAM,
    hram: RAM,
    eram: RAM,
    oam: RAM,
    io: Io,
}

impl Bus {
    pub fn new(mbc: Mbc, timer: Rc<RefCell<timer::Timer>>) -> Self {
        Bus {
            mbc,
            vram: RAM::new(0x2000),
            wram: RAM::new(0x4000),
            wram2: RAM::new(0x4000),
            hram: RAM::new(0x0080),
            eram: RAM::new(0x2000),
            oam: RAM::new(0x00A0),
            io: Io::new(timer),
        }
    }
}

impl Reader for Bus {
    fn read(&self, addr: Word) -> Byte {
        match addr {
            0x0000..=0x7FFF => self.mbc.read(addr),
            0x8000..=0x9FFF => self.vram.read(addr - 0x8000),
            0xA000..=0xBFFF => self.mbc.read(addr),
            0xC000..=0xCFFF => self.wram.read(addr - 0xC000),
            0xD000..=0xDFFF => self.wram2.read(addr - 0xD000),
            0xE000..=0xFDFF => self.eram.read(addr - 0xE000),
            0xFE00..=0xFE9F => self.oam.read(addr - 0xFE00),
            0xFEA0..=0xFEFF => 0,
            0xFF00..=0xFF70 | 0xFFFF => self.io.read(addr),
            0xFF80..=0xFFFE => self.hram.read(addr - 0xFF80),
            v => todo!("addr {:04X} is not readable", v),
        }
    }
}

impl Writer for Bus {
    fn write(&mut self, addr: Word, value: Byte) {
        match addr {
            0x0000..=0x7FFF => self.mbc.write(addr, value),
            0x8000..=0x9FFF => self.vram.write(addr - 0x8000, value),
            0xA000..=0xBFFF => self.mbc.write(addr, value),
            0xC000..=0xCFFF => self.wram.write(addr - 0xC000, value),
            0xD000..=0xDFFF => self.wram2.write(addr - 0xD000, value),
            0xE000..=0xFDFF => self.eram.write(addr - 0xE000, value),
            0xFE00..=0xFE9F => self.oam.write(addr - 0xFE00, value),
            0xFEA0..=0xFEFF => (),
            0xFF00..=0xFF70 | 0xFFFF => self.io.write(addr, value),
            0xFF80..=0xFFFE => self.hram.write(addr - 0xFF80, value),
            v => todo!("addr {:04X} is not writable", v),
        }
    }
}

impl BusAccessor for Bus {
    fn interrupt(&mut self) -> &mut interrupt::Interrupt {
       &mut self.io.interrupt 
    }
}