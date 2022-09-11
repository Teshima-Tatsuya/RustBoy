use crate::{constant::*, traits::*, types::*};

#[derive(Default)]
pub struct Apu {
    nr10: Byte,
    nr11: Byte,
    nr12: Byte,
    nr13: Byte,
    nr14: Byte,
    nr21: Byte,
    nr22: Byte,
    nr23: Byte,
    nr24: Byte,
    nr30: Byte,
    nr31: Byte,
    nr32: Byte,
    nr33: Byte,
    nr34: Byte,
    nr41: Byte,
    nr42: Byte,
    nr43: Byte,
    nr44: Byte,
    nr50: Byte,
    nr51: Byte,
    nr52: Byte,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Reader for Apu {
    fn read(&self, addr: Word) -> Byte {
        match addr {
            ADDR_APU_NR10 => self.nr10 | 0x80,
            ADDR_APU_NR11 => self.nr11,
            ADDR_APU_NR12 => self.nr12,
            ADDR_APU_NR13 => self.nr13,
            ADDR_APU_NR14 => self.nr14,
            ADDR_APU_NR21 => self.nr21,
            ADDR_APU_NR22 => self.nr22,
            ADDR_APU_NR23 => self.nr23,
            ADDR_APU_NR24 => self.nr24,
            ADDR_APU_NR30 => self.nr30 | 0x7F,
            ADDR_APU_NR31 => self.nr31,
            ADDR_APU_NR32 => self.nr32 | 0x9F,
            ADDR_APU_NR33 => self.nr33,
            ADDR_APU_NR34 => self.nr34,
            ADDR_APU_NR41 => self.nr41 | 0xC0,
            ADDR_APU_NR42 => self.nr42,
            ADDR_APU_NR43 => self.nr43,
            ADDR_APU_NR44 => self.nr44 | 0x3F,
            ADDR_APU_NR50 => self.nr50,
            ADDR_APU_NR51 => self.nr51,
            ADDR_APU_NR52 => self.nr52 | 0x70,
            _ => 0xFF,
        }
    }
}

impl Writer for Apu {
    fn write(&mut self, addr: Word, value: Byte) {
        match addr {
            ADDR_APU_NR10 => self.nr10 = value,
            ADDR_APU_NR11 => self.nr11 = value,
            ADDR_APU_NR12 => self.nr12 = value,
            ADDR_APU_NR13 => self.nr13 = value,
            ADDR_APU_NR14 => self.nr14 = value,
            ADDR_APU_NR21 => self.nr21 = value,
            ADDR_APU_NR22 => self.nr22 = value,
            ADDR_APU_NR23 => self.nr23 = value,
            ADDR_APU_NR24 => self.nr24 = value,
            ADDR_APU_NR30 => self.nr30 = value,
            ADDR_APU_NR31 => self.nr31 = value,
            ADDR_APU_NR32 => self.nr32 = value,
            ADDR_APU_NR33 => self.nr33 = value,
            ADDR_APU_NR34 => self.nr34 = value,
            ADDR_APU_NR41 => self.nr41 = value,
            ADDR_APU_NR42 => self.nr42 = value,
            ADDR_APU_NR43 => self.nr43 = value,
            ADDR_APU_NR44 => self.nr44 = value,
            ADDR_APU_NR50 => self.nr50 = value,
            ADDR_APU_NR51 => self.nr51 = value,
            ADDR_APU_NR52 => self.nr52 = value,
            _ => (),
        }
        
    }
}
