use std::{
    cell::RefCell,
    rc::Rc,
};

use crate::{
    constant::*,
    traits::*,
    types::*,
    interrupt::Interrupt,
};

pub struct Timer {
    counter: u16,
    div: Byte,
    tima: Byte,
    tma: Byte,
    tac: Byte,
    interrupt: Rc<RefCell<Interrupt>>,
}

impl Timer {
    pub fn new(interrupt: Rc<RefCell<Interrupt>>) -> Self {
        Self {
            div: 0x19,
            interrupt: interrupt,
            counter: 0,
            tima: 0x00,
            tma: 0x00,
            tac: 0x00,
        }
    }

    pub fn tick(&mut self, cycle: u16) {
        for _ in 0..cycle {
            self.counter = self.counter.wrapping_add(4);

            if self.counter % 256 == 0 {
                self.div = self.div.wrapping_add(1);
            }

            if !self.started() {
                continue;
            }

            if self.tima == 0 {
                self.tima = self.tma;
                self.interrupt.borrow_mut().request(INT_TIMER_FLG);
            }

            if self.counter % (1 << (self.get_freq() + 1)) == 0 {
                self.tima = self.tima.wrapping_add(1);
            }
        }
    }

    fn get_freq(&self) -> u16 {
        match self.tac & 0x03 {
            0 => 9,
            1 => 3,
            2 => 5,
            3 => 7,
            v => unreachable!("Illegal TAC with {}", v),
        }
    }

    fn started(&self) -> bool {
        self.tac & 0x04 == 0x04
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
            ADDR_TIMER_DIV => {
                self.div = 0;
                if self.counter >> self.get_freq() & 0x01 == 1 {
                    self.tima = self.tima.wrapping_add(1);
                }
                self.counter = 0;
            },
            ADDR_TIMER_TIMA => self.tima = value,
            ADDR_TIMER_TMA => self.tma = value,
            ADDR_TIMER_TAC => self.tac = value & 0x07,
            v => unreachable!("Non Supported addr {:04X}", v),
        }
    }
}