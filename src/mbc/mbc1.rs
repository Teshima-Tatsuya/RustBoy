use crate::cartridge::Cartridge;
use crate::traits::*;
use crate::types::*;

const SIMPLE_ROMBANKING_MODE: u8 = 0x00;
const RAMBANKING_MODE_ADVANCED_ROMBANKING_MODE: u8 = 0x01;

pub struct Mbc1 {
    cartridge: Cartridge,
    rom_bank: u8,
    ram_bank: u8,
    ram_enable: bool,
    mode: u8,
}

impl Mbc1 {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge: cartridge,
            rom_bank: 1,
            ram_bank: 0,
            ram_enable: false,
            mode: 0x00,
        }
    }
}

impl super::MbcTrait for Mbc1 {
    fn read(&self, addr: Word) -> Byte {
        self.cartridge.rom.read(addr)
    }

    fn write(&mut self, addr: Word, value: Byte) {
        // @see https://gbdev.io/pandocs/MBC1.html
        // Implement Write address range
        match addr {
            0x0000..=0x1FFF => {
                if value == 0x00 {
                    self.ram_enable = false
                } else if value == 0x0A {
                    self.ram_enable = true
                }
            }
            0x2000..=0x3FFF => {
                self.switch_rom_bank((value & 0x1F) as u16);
            }
            0x4000..=0x5FFF => {
                if self.mode == SIMPLE_ROMBANKING_MODE {
                    self.switch_hi_rom_bank(value);
                } else if self.mode == RAMBANKING_MODE_ADVANCED_ROMBANKING_MODE {
                    // lower 2bit
                    self.switch_ram_bank((value & 0x03) as u16);
                }
            }
            0x6000..=0x7FFF => self.mode = value,
            0xA000..=0xC000 => {
                if self.ram_enable {
                    let computed_addr = addr + (self.ram_bank as u16) * 0x2000 - 0xA000;
                    self.cartridge.ram.write(computed_addr, value)
                }
            }
            v => unreachable!("{}", v),
        }
    }

    fn switch_rom_bank(&mut self, bank: u16) {
        let mut bank2 = bank;
        if bank == 0x00 || bank == 0x20 || bank == 0x40 || bank == 0x60 {
            bank2 += 1;
        }

        self.rom_bank = bank2 as u8;
    }

    fn switch_ram_bank(&mut self, bank: u16) {
        self.ram_bank = bank as u8;
    }
}

impl Mbc1 {
    fn switch_hi_rom_bank(&mut self, value: Byte) {
        // clear Hi bit
        self.rom_bank &= 0x1F;

        // clear Low bit
        let value2 = value & 0xE0;

        self.rom_bank |= value2;
    }
}
