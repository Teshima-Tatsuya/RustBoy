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
pub const ADDR_APU_NR10: Word = 0xFF10;
pub const ADDR_APU_NR11: Word = 0xFF11;
pub const ADDR_APU_NR12: Word = 0xFF12;
pub const ADDR_APU_NR13: Word = 0xFF13;
pub const ADDR_APU_NR14: Word = 0xFF14;
pub const ADDR_APU_NR21: Word = 0xFF16;
pub const ADDR_APU_NR22: Word = 0xFF17;
pub const ADDR_APU_NR23: Word = 0xFF18;
pub const ADDR_APU_NR24: Word = 0xFF19;
pub const ADDR_APU_NR30: Word = 0xFF1A;
pub const ADDR_APU_NR31: Word = 0xFF1B;
pub const ADDR_APU_NR32: Word = 0xFF1C;
pub const ADDR_APU_NR33: Word = 0xFF1D;
pub const ADDR_APU_NR34: Word = 0xFF1E;
pub const ADDR_APU_NR41: Word = 0xFF20;
pub const ADDR_APU_NR42: Word = 0xFF21;
pub const ADDR_APU_NR43: Word = 0xFF22;
pub const ADDR_APU_NR44: Word = 0xFF23;
pub const ADDR_APU_NR50: Word = 0xFF24;
pub const ADDR_APU_NR51: Word = 0xFF25;
pub const ADDR_APU_NR52: Word = 0xFF26;
pub const ADDR_PPU_LCDC: Word = 0xFF40;
pub const ADDR_PPU_LCDS: Word = 0xFF41;
pub const ADDR_PPU_SCY: Word = 0xFF42;
pub const ADDR_PPU_SCX: Word = 0xFF43;
pub const ADDR_PPU_LY: Word = 0xFF44;
pub const ADDR_PPU_LYC: Word = 0xFF45;
pub const ADDR_PPU_DMA: Word = 0xFF46;
pub const ADDR_PPU_BGP: Word = 0xFF47;
pub const ADDR_PPU_OBP0: Word = 0xFF48;
pub const ADDR_PPU_OBP1: Word = 0xFF49;
pub const ADDR_PPU_WY: Word = 0xFF4A;
pub const ADDR_PPU_WX: Word = 0xFF4B;
pub const ADDR_PPU_BCPS: Word = 0xFF68;
pub const ADDR_PPU_BCPD: Word = 0xFF69;
pub const ADDR_PPU_OCPS: Word = 0xFF6A;
pub const ADDR_PPU_OCPD: Word = 0xFF6B;
