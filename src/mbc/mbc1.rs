use crate::cartridge::Cartridge;
use crate::types::*;

pub struct Mbc1 {
    cartridge: Cartridge,
}

impl Mbc1 {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge: cartridge,
        }
    }
}

impl super::MbcTrait for Mbc1 {
    fn read(&self, addr: Word) -> Byte {
        0
    }
}
