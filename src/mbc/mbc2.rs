use crate::cartridge::Cartridge;
use crate::types::*;

// https://gekkio.fi/files/gb-docs/gbctr.pdf
pub struct Mbc2 {
    cartridge: Cartridge,
    rom_bank: u8,
    rom_bank_mask: u8,
    ram_enable: bool,
}

impl Mbc2 {
    pub fn new(cartridge: Cartridge) -> Self {
        let rom_bank_mask = ((cartridge.rom_size / 0x4000) as u8).saturating_sub(1);

        Self {
            cartridge: cartridge,
            rom_bank: 1,
            rom_bank_mask,
            ram_enable: false,
        }
    }
}

impl super::MbcTrait for Mbc2 {
    fn read(&self, addr: Word) -> Byte {
        match addr {
            0..=0x3FFF => self.cartridge.rom.buf[(addr & 0x1FFF) as usize],
            0x4000..=0x7FFF => {
                let rom_bank = self.rom_bank & self.rom_bank_mask;
                self.cartridge.rom.buf[((addr & 0x3FFF) as usize)
                    .wrapping_add((rom_bank as usize).wrapping_mul(0x4000))]
            }
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    let computed_addr = (addr & 0x01FF) as usize; // 0xA000 = 0xA200 = 0xA400 .. and so on.
                    self.cartridge.ram.buf[computed_addr] & 0x0F // only lower 4-bit
                } else {
                    0xFF
                }
            }
            v => {
                log::warn!("MBC2 doesn't support read addr:0x{:04X}", v);
                0xFF
            }
        }
    }

    fn write(&mut self, addr: Word, value: Byte) {
        // @see https://gbdev.io/pandocs/MBC2.html
        // @see https://gekkio.fi/files/gb-docs/gbctr.pdf MBC2 mapper chip
        match addr {
            0x0000..=0x3FFF => {
                if addr & 0x0100 != 0 {
                    if value & 0x0F == 0x00 {
                        self.ram_enable = false
                    } else if value & 0x0F == 0x0A {
                        self.ram_enable = true
                    }
                } else {
                    self.switch_rom_bank((value & 0x0F) as u16);
                }
            },
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    let computed_addr = (addr & 0x01FF) as usize; // 0xA000 = 0xA200 = 0xA400 .. and so on.
                    self.cartridge.ram.buf[computed_addr] = value & 0x0F; // only lower 4-bit
                }
            }
            v => log::warn!("MBC2 doesn't support write addr:0x{:04X}", v),
        }
    }

    fn switch_rom_bank(&mut self, bank: u16) {
        let mut bank2 = bank;
        if bank == 0x00 {
            bank2 += 1;
        }

        self.rom_bank = bank2 as u8;
    }
}
