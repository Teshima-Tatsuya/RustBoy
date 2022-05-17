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

    fn write(&mut self, addr: Word, value: Byte) {
        unreachable!();
    }

    fn switch_rom_bank(&mut self, bank: u16) {
        unreachable!();
    }

    fn switch_ram_bank(&mut self, bank: u16) {
        unreachable!();
    }
}
