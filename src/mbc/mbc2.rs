use crate::cartridge::Cartridge;
use crate::traits::*;
use crate::types::*;

const SIMPLE_ROMBANKING_MODE: u8 = 0x00;
const RAMBANKING_MODE_ADVANCED_ROMBANKING_MODE: u8 = 0x01;

// https://gekkio.fi/files/gb-docs/gbctr.pdf
pub struct Mbc2 {
    cartridge: Cartridge,
    rom_bank: u8,
    rom_bank_mask: u8,
    ram_enable: bool,
    mode: u8,
}

// TODO multicart
impl Mbc2 {
    pub fn new(cartridge: Cartridge) -> Self {
        let rom_bank_mask = ((cartridge.rom_size / 0x4000) as u8).saturating_sub(1);
        let ram_bank_mask = ((cartridge.ram_size / 0x2000) as u8).saturating_sub(1);

        Self {
            cartridge: cartridge,
            rom_bank: 1,
            rom_bank_mask,
            ram_enable: false,
            mode: SIMPLE_ROMBANKING_MODE,
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
                    let computed_addr = ((addr & 0x1FFF) as usize);
                    self.cartridge.ram.buf[computed_addr]
                } else {
                    0xFF
                }
            }
            v => unreachable!("{}", v),
        }
    }

    fn write(&mut self, addr: Word, value: Byte) {
        // @see https://gbdev.io/pandocs/MBC1.html
        // Implement Write address range
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
                    let computed_addr = ((addr & 0x1FFF) as usize);

                    self.cartridge.ram.buf[computed_addr] = value;
                }
            }
            v => unreachable!("{}", v),
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
