use crate::cartridge::Cartridge;
use crate::traits::Reader;
use crate::types::*;

pub struct NoMbc {
    cartridge: Cartridge,
}

impl NoMbc {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge: cartridge,
        }
    }
}

impl super::MbcTrait for NoMbc {
    fn read(&mut self, addr: Word) -> Byte {
        self.cartridge.rom.read(addr)
    }
}
