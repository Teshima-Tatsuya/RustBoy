use crate::cartridge::Cartridge;
use crate::types::*;

// https://gekkio.fi/files/gb-docs/gbctr.pdf
pub struct Mbc5 {
    cartridge: Cartridge,
    rom_bank: u8,
    rom_bank_high: u8,
    rom_bank_mask: u8,
    ram_bank: u8,
    ram_bank_mask: u8,
    ram_enable: bool,
}

impl Mbc5 {
    pub fn new(cartridge: Cartridge) -> Self {
        let rom_bank_mask = ((cartridge.rom_size / 0x4000) as u8).saturating_sub(1);
        let ram_bank_mask = ((cartridge.ram_size / 0x2000) as u8).saturating_sub(1);

        Self {
            cartridge: cartridge,
            rom_bank: 1,
            rom_bank_high: 0,
            rom_bank_mask,
            ram_bank: 0,
            ram_bank_mask,
            ram_enable: false,
        }
    }
}

impl super::MbcTrait for Mbc5 {
    fn read(&self, addr: Word) -> Byte {
        match addr {
            0..=0x3FFF => self.cartridge.rom.buf[(addr & 0x1FFF) as usize],
            0x4000..=0x7FFF => {
                let rom_bank = (
                     ((self.rom_bank_high as usize) << 8) |
                     (self.rom_bank as usize)
                    ) & self.rom_bank_mask as usize;
                self.cartridge.rom.buf[((addr & 0x3FFF) as usize)
                    .wrapping_add((rom_bank as usize).wrapping_mul(0x4000))]
            }
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    let computed_addr = ((addr & 0x1FFF) as usize).wrapping_add(((self.ram_bank & self.ram_bank_mask) as usize).wrapping_mul(0x2000));

                    self.cartridge.ram.buf[computed_addr]
                } else {
                    0xFF
                }
            },
            v => {
                log::warn!("MBC5 doesn't support read addr:0x{:04X}", v);
                0xFF
            }
        }
    }

    fn write(&mut self, addr: Word, value: Byte) {
        // @see https://gbdev.io/pandocs/MBC1.html
        // Implement Write address range
        match addr {
            0x0000..=0x1FFF => {
                self.ram_enable = value == 0x0A;
            },
            0x2000..=0x2FFF => {
                self.rom_bank = value;
            },
            0x3000..=0x3FFF => {
                self.rom_bank_high = value & 0x01;
            },
            0x4000..=0x5FFF => {
                self.ram_bank= value & 0x0F;
            },
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    let computed_addr = ((addr & 0x1FFF) as usize).wrapping_add(((self.ram_bank & self.ram_bank_mask) as usize).wrapping_mul(0x2000));

                    self.cartridge.ram.buf[computed_addr] = value;
                }
            }
            v => log::warn!("MBC5 doesn't support write addr:0x{:04X}", v)
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
