use crate::{
    types::*,
    traits::*,
    constant::*,
};

#[derive(Default)]
pub struct Serial {
    sb: Byte,

    /// Bit 7 - Transfer Start Flag (0=No transfer is in progress or requested, 1=Transfer in progress, or requested)  
    /// Bit 1 - Clock Speed (0=Normal, 1=Fast) ** CGB Mode Only **  
    /// Bit 0 - Shift Clock (0=External Clock, 1=Internal Clock)  
    sc: Byte,
}

impl Serial {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Reader for Serial {
    fn read(&self, addr: Word) -> Byte {
        match addr {
           ADDR_SB => self.sb,
           ADDR_SC => self.sc | 0x7E,
           v => unreachable!("Invalid Addr {:04X} for Serial", v),
        }
    }
}

impl Writer for Serial {
    fn write(&mut self, addr: Word, value: Byte) {
        match addr {
           ADDR_SB => self.sb = value,
           ADDR_SC => self.sc = value & 0x83,
           v => unreachable!("Invalid Addr {:04X} for Serial", v),
        }
    }
}