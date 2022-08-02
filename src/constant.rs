use crate::types::*;

// reg
pub const R_ARR: [&str; 8] = ["A", "B", "C", "D", "E", "F", "H", "L"];
pub const RR_ARR: [&str; 6] = ["AF", "BC", "DE", "HL", "PC", "SP"];
pub const M_ARR: [&str; 1] = ["(C)"];
pub const MM_ARR: [&str; 4] = ["(BC)", "(DE)", "(HL)", "(AF)"];
pub const COND_ARR: [&str; 4] = ["Z", "NZ", "C", "NC"];

// screen
pub const SCREEN_WIDTH: u8 = 160;
pub const SCREEN_HEIGHT: u8 = 144;

// interrupt
pub const INT_VBLANK: u8 = 0;
pub const INT_LCD_STAT: u8 = 1;
pub const INT_TIMER: u8 = 2;
pub const INT_SERIAL: u8 = 3;
pub const INT_JOYPAD: u8 = 4;

// ADDR
pub const ADDR_JOYPAD: Word = 0xFF00;
pub const ADDR_SERIAL_SB: Word = 0xFF01;
pub const ADDR_SERIAL_SC: Word = 0xFF02;
pub const ADDR_TIMER_DIV: Word = 0xFF04;
pub const ADDR_TIMER_TIMA: Word = 0xFF05;
pub const ADDR_TIMER_TMA: Word = 0xFF06;
pub const ADDR_TIMER_TAC: Word = 0xFF07;
pub const ADDR_INTERRUPT_IF: Word = 0xFF0F;
pub const ADDR_INTERRUPT_IE: Word = 0xFFFF;