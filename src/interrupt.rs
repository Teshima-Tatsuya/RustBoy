use crate::{
    types::*,
    constant::*,
    traits::*,
};
use std::fmt;

#[derive(Default)]
pub struct Interrupt {
    pub r#if: Byte,
    pub ie: Byte,
}

impl fmt::Display for Interrupt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Interrupt if:0x{:02X} ie:0x{:02X}",
            self.r#if, self.ie
        )
    }
}

impl Interrupt {
    pub fn new() -> Self {
        Self {
            r#if:0x00,
            ie: 0x00,
        }
    }

    pub fn has(&self) -> bool {
        (self.r#if & self.ie) != 0x00
    }

    pub fn request(&mut self, value: Byte) {
        self.write(ADDR_INTERRUPT_IF, value);
    }

    // @see https://gbdev.io/pandocs/interrupts.html#interrupt-priorities
    pub fn interrupt_addr(&mut self) -> Word{
        let idx = self.r#if & self.ie;

        if idx & INT_VBLANK_FLG != 0 {
            self.r#if ^= INT_VBLANK_FLG;
            return INT_VBLANK_ADDR;
        } else if idx & INT_LCD_STAT_FLG != 0 {
            self.r#if ^= INT_LCD_STAT_FLG;
            return INT_LCD_STAT_ADDR;
        } else if idx & INT_TIMER_FLG != 0 {
            self.r#if ^= INT_TIMER_FLG;
            return INT_TIMER_ADDR;
        } else if idx & INT_SERIAL_FLG != 0 {
            self.r#if ^= INT_SERIAL_FLG;
            return INT_SERIAL_ADDR;
        } else if idx & INT_JOYPAD_FLG != 0 {
            self.r#if ^= INT_JOYPAD_FLG;
            return INT_JOYPAD_ADDR;
        } else {
            unreachable!("idx {:02X} is invalid for interrupt!!", idx);
        }
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
            ADDR_INTERRUPT_IF => self.r#if |= value & 0x1F,
            v => unreachable!("Invalid Addr {:04X} for Interrupt", v),
        }
    }
}