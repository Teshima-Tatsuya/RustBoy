use crate::cartridge::Cartridge;
use crate::types::*;

mod mbc1;
mod no_mbc;

pub trait MbcTrait {
    fn read(&self, addr: Word) -> Byte;
    fn write(&mut self, addr: Word, value: Byte);
    fn switch_rom_bank(&mut self, bank: u16);
    fn switch_ram_bank(&mut self, bank: u16);
}

pub enum Mbc {
    NoMbc(no_mbc::NoMbc),
}

pub fn new_mbc(cartridge: Cartridge) -> Mbc {
    Mbc::NoMbc(no_mbc::NoMbc::new(cartridge))
}
