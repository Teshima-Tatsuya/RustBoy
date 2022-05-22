mod mbc1;
mod no_mbc;

use crate::cartridge::Cartridge;
use crate::types::*;
use ambassador::{delegatable_trait, Delegate};

#[delegatable_trait]
pub trait MbcTrait {
    fn read(&self, addr: Word) -> Byte;
    fn write(&mut self, addr: Word, value: Byte) {}
    fn switch_rom_bank(&mut self, bank: u16) {}
    fn switch_ram_bank(&mut self, bank: u16) {}
}

#[derive(Delegate)]
#[delegate(MbcTrait)]
pub enum Mbc {
    NoMbc(no_mbc::NoMbc),
    Mbc1(mbc1::Mbc1),
}

pub fn new_mbc(cartridge: Cartridge) -> Mbc {
    Mbc::NoMbc(no_mbc::NoMbc::new(cartridge))
}
