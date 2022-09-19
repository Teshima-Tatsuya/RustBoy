use std::{
    sync::{
        Arc,
        Mutex,
    }
};

use crate::{
    constant::*,
    traits::*,
    types::*,
    interrupt::Interrupt,
};

#[derive(Default)]
pub struct Timer {
    counter: u16,
    div: Byte,
    tima: Byte,
    tima_ovewflowed: bool,
    tma: Byte,
    tac: Byte,
    interrupt: Arc<Mutex<Interrupt>>,
}

impl std::fmt::Display for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Timer: counter:{:04X} div:{:02X} tima:{:02X} tma:{:02X} tac:{:02X}",
            self.counter, self.div, self.tima, self.tma, self.tac
        )
    }

}

impl Timer {
    pub fn new(interrupt: Arc<Mutex<Interrupt>>) -> Self {
        Self {
            interrupt: interrupt,
            ..Default::default()
        }
    }

    pub fn tick(&mut self, cycle: u16) {
        for _ in 0..cycle {
            log::trace!("{}", self);
            self.counter = self.counter.wrapping_add(4);

            if self.counter % 256 == 0 {
                self.div = self.div.wrapping_add(1);
            }

            if !self.started() {
                continue;
            }

            // tima overflow
            self.tima_ovewflowed = false;
            if self.tima == 0 {
                self.tima = self.tma;
                self.interrupt.lock().unwrap().request(INT_TIMER_FLG);
                self.tima_ovewflowed = true;
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
            ADDR_TIMER_TIMA => {
                if !self.tima_ovewflowed {
                    self.tima = value;
                }
            },
            ADDR_TIMER_TMA => {
                self.tma = value;
                if self.tima_ovewflowed {
                    self.tima = value;
                }
            },
            ADDR_TIMER_TAC => self.tac = value & 0x07,
            v => unreachable!("Non Supported addr {:04X}", v),
        }
    }
}