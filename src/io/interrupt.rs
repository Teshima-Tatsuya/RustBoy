use crate::{
    types::*,
    constant::*,
    traits::*,
};

pub struct Interrupt {
    r#if: Byte,
    ie: Byte,
    ime: bool,
}

impl Interrupt {
    pub fn new() -> Self {
        Self {
            r#if:0x00,
            ie: 0x00,
            ime: false,
        }
    }

    pub fn has(&self) -> bool {
        (self.r#if & self.ie) != 0x00
    }
    
    pub fn enable(&mut self) {
        self.ime = true;
    }

    pub fn disable(&mut self) {
        self.ime = false;
    }
}

impl Reader for Interrupt {
    fn read(&self, addr: Word) -> Byte {
        match addr {
            ADDR_INTERRUPT_IE => self.ie,
            ADDR_INTERRUPT_IF => self.r#if | 0xE0,
            v => unreachable!("Invalid Addr {:04X} for Interrupt", v),
        }
    }
}

impl Writer for Interrupt {
    fn write(&mut self, addr: Word, value: Byte) {
        match addr {
            ADDR_INTERRUPT_IE => self.ie = value,
            ADDR_INTERRUPT_IF => self.r#if = value & 0x1F,
            v => unreachable!("Invalid Addr {:04X} for Interrupt", v),
        }
    }
}