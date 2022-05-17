use crate::types::*;

mod mbc1;
mod no_mbc;

pub trait Mbc {
    fn read(&self, addr: Word) -> Byte;
    fn write(&mut self, addr: Word, value: Byte);
    fn switch_rom_bank(&mut self, bank: u16);
    fn switch_ram_bank(&mut self, bank: u16);
}
