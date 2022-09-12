mod mbc1;
mod mbc2;
mod mbc3;
mod mbc5;
mod no_mbc;

use crate::cartridge::Cartridge;
use crate::types::*;
use ambassador::{delegatable_trait, Delegate};

#[delegatable_trait]
pub trait MbcTrait {
    fn read(&self, addr: Word) -> Byte;
    fn write(&mut self, _addr: Word, _value: Byte) {}
    fn switch_rom_bank(&mut self, _bank: u16) {}
    fn switch_ram_bank(&mut self, _bank: u16) {}
}

#[derive(Delegate)]
#[delegate(MbcTrait)]
pub enum Mbc {
    NoMbc(no_mbc::NoMbc),
    Mbc1(mbc1::Mbc1),
    Mbc2(mbc2::Mbc2),
    Mbc3(mbc3::Mbc3),
    Mbc5(mbc5::Mbc5),
}

pub fn new_mbc(cartridge: Cartridge) -> Mbc {
    match cartridge.cartridge_type.mbc {
        Some(crate::cartridge::Mbc::Mbc1) => Mbc::Mbc1(mbc1::Mbc1::new(cartridge)),
        Some(crate::cartridge::Mbc::Mbc2) => Mbc::Mbc2(mbc2::Mbc2::new(cartridge)),
        Some(crate::cartridge::Mbc::Mbc3) => Mbc::Mbc3(mbc3::Mbc3::new(cartridge)),
        Some(crate::cartridge::Mbc::Mbc5) => Mbc::Mbc5(mbc5::Mbc5::new(cartridge)),
        None => Mbc::NoMbc(no_mbc::NoMbc::new(cartridge)),
        Some(v) => todo!("type {} hasn't implemented", v),
    }
}
