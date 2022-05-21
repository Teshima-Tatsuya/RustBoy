use crate::cartridge::Cartridge;
use crate::traits::Reader;
use crate::types::*;

#[derive(Default)]
pub struct NoMbc {
    cartridge: Cartridge,
}

impl NoMbc {
    pub fn new(cartridge: Cartridge) -> Self {
        NoMbc {
            cartridge: cartridge,
        }
    }
}

impl super::MbcTrait for NoMbc {
    fn read(&self, addr: Word) -> Byte {
        self.cartridge.rom.read(addr)
    }
}
