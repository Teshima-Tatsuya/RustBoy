use crate::{constant::*, traits::*, types::*};

#[derive(Default)]
pub struct Timer {
    div: Byte,
    tima: Byte,
    tma: Byte,
    tac: Byte,
}

impl Timer {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Reader for Timer {
    fn read(&self, addr: Word) -> Byte {
        match addr {
            ADDR_TIMER_DIV => self.div,
            ADDR_TIMER_TIMA => self.tima,
            ADDR_TIMER_TMA => self.tma,
            ADDR_TIMER_TAC => self.tac | 0xF8,
            v => unreachable!("Non Supported addr {:04X}", v),
        }
    }
}

impl Writer for Timer {
    fn write(&mut self, addr: Word, value: Byte) {
        match addr {
            ADDR_TIMER_DIV => self.div = value,
            ADDR_TIMER_TIMA => self.tima = value,
            ADDR_TIMER_TMA => self.tma = value,
            ADDR_TIMER_TAC => self.tac = value & 0x07,
            v => unreachable!("Non Supported addr {:04X}", v),
        }
    }
}
