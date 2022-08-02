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
    io: Io,
}

impl Bus {
    pub fn new(mbc: Mbc) -> Self {
        Bus {
            mbc,
            vram: RAM::new(0x2000),
            wram: RAM::new(0x4000),
            wram2: RAM::new(0x4000),
            hram: RAM::new(0x0080),
            io: Io::new(),
        }
    }
}

impl Reader for Bus {
    fn read(&self, addr: Word) -> Byte {
        match addr {
            0x0000..=0x7FFF => self.mbc.read(addr),
            0x8000..=0x9FFF => self.vram.read(addr - 0x8000),
            0xC000..=0xDFFF => self.wram.read(addr - 0xC000),
            0xE000..=0xFDFF => self.wram2.read(addr - 0xE000),
            0xFF01..=0xFF70 | 0xFFFF => self.io.read(addr),
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
            0xC000..=0xDFFF => self.wram.write(addr - 0xC000, value),
            0xE000..=0xFDFF => self.wram2.write(addr - 0xE000, value),
            0xFF01..=0xFF70 | 0xFFFF => self.io.write(addr, value),
            0xFF80..=0xFFFE => self.hram.write(addr - 0xFF80, value),
            v => todo!("addr {:04X} is not writable", v),
        }
    }
}
