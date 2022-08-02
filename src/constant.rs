use crate::types::*;

pub const R_ARR: [&str; 8] = ["A", "B", "C", "D", "E", "F", "H", "L"];
pub const RR_ARR: [&str; 6] = ["AF", "BC", "DE", "HL", "PC", "SP"];
pub const M_ARR: [&str; 1] = ["(C)"];
pub const MM_ARR: [&str; 4] = ["(BC)", "(DE)", "(HL)", "(AF)"];
pub const COND_ARR: [&str; 4] = ["Z", "NZ", "C", "NC"];

// ADDR
pub const ADDR_JOYPAD: Word = 0xFF00;
pub const ADDR_SERIAL_SB: Word = 0xFF01;
pub const ADDR_SERIAL_SC: Word = 0xFF02;
pub const ADDR_TIMER_DIV: Word = 0xFF04;
pub const ADDR_TIMER_TIMA: Word = 0xFF05;
pub const ADDR_TIMER_TMA: Word = 0xFF06;
pub const ADDR_TIMER_TAC: Word = 0xFF07;