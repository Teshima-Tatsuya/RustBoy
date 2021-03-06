use crate::traits::*;
use crate::types::*;
use crate::{mbc::Mbc, mbc::MbcTrait};

pub struct Bus {
    mbc: Mbc,
}

impl Bus {
    pub fn new(mbc: Mbc) -> Self {
        Bus { mbc }
    }
}

impl Reader for Bus {
    fn read(&self, addr: Word) -> Byte {
        match addr {
            0x0000..=0x7FFF => self.mbc.read(addr),
            _ => 0x00,
        }
    }
}

impl Writer for Bus {
    fn write(&mut self, addr: Word, value: Byte) {
        match addr {
            0x0000..=0x7FFF => self.mbc.write(addr, value),
            v => todo!("addr {} is not writable", v),
        }
    }
}
