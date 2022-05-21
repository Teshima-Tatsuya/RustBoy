use crate::traits::*;
use crate::types::*;
use crate::{mbc::Mbc, mbc::MbcTrait};

struct Bus {
    mbc: Mbc,
}

impl Bus {
    fn new(mbc: Mbc) -> Self {
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
