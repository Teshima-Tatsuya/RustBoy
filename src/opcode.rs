use crate::{constant::*, cpu::Cpu, types::*, util::*, traits::*};
use once_cell::sync::Lazy;
use std::fmt;

pub struct OpCode {
	pub code: Byte,
	pub mnemonic: String,
	pub r1: String,
	pub r2: String,
	pub cycles: u8,
	pub handler: fn(c: &mut Cpu, String, String),
}

impl fmt::Display for OpCode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"OpCode: 0x{:02X}[{}] cycles:{}",
			self.code, self.mnemonic, self.cycles
		)
	}
}

macro_rules! make_opcode {
	($code:expr, $mnemonic:expr, $r1:expr, $r2:expr, $cycle:expr, $handler:expr) => {
		OpCode {
			code: $code,
			mnemonic: String::from($mnemonic),
			r1: String::from($r1),
			r2: String::from($r2),
			cycles: $cycle,
			handler: $handler,
		}
	};
}

#[rustfmt::skip]
pub static OPCODES: Lazy<[OpCode; 256]> = Lazy::new(|| [
    make_opcode! {0x00, "NOP", "", "", 1, nop},
	make_opcode! {0x01, "LD BC,d16", "BC", "dd",   3, ld},
	make_opcode! {0x02, "LD (BC),A", "(BC)", "A",  2, ld},
	make_opcode! {0x03, "INC BC", "BC", "",   2, inc},
	make_opcode! {0x04, "INC B", "B", "",   1, inc},
	make_opcode! {0x05, "DEC B", "B", "",   1, dec},
	make_opcode! {0x06, "LD B,d8", "B", "d",   2, ld},
	make_opcode! {0x07, "RLCA", "",  "",   1, rlca},
	make_opcode! {0x08, "LD (a16),SP", "(aa)",  "SP",  5, ld},
	make_opcode! {0x09, "ADD HL,BC", "HL", "BC",  2, addr16r16},
	make_opcode! {0x0A, "LD A,(BC)", "A", "(BC)",  2, ld},
	make_opcode! {0x0B, "DEC BC", "BC", "",   2, dec},
	make_opcode! {0x0C, "INC C", "C", "",   1, inc},
	make_opcode! {0x0D, "DEC C", "C", "",   1, dec},
	make_opcode! {0x0E, "LD C,d8", "C", "d",   2, ld},
	make_opcode! {0x0F, "RRCA", "",  "",   1, rrca},
	make_opcode! {0x10, "STOP 0", "",  "",   1, stop},
	make_opcode! {0x11, "LD DE,d16", "DE", "dd",   3, ld},
	make_opcode! {0x12, "LD (DE),A", "(DE)", "A",  2, ld},
	make_opcode! {0x13, "INC DE", "DE", "",   2, inc},
	make_opcode! {0x14, "INC D", "D", "",   1, inc},
	make_opcode! {0x15, "DEC D", "D", "",   1, dec},
	make_opcode! {0x16, "LD D,d8", "D", "d",   2, ld},
	make_opcode! {0x17, "RLA", "",  "",   1, rla},
	make_opcode! {0x18, "JR r8", "d",  "",   3, jr},
	make_opcode! {0x19, "ADD HL,DE", "HL", "DE",  2, addr16r16},
	make_opcode! {0x1A, "LD A,(DE)", "A", "(DE)",  2, ld},
	make_opcode! {0x1B, "DEC DE", "DE", "",   2, dec},
	make_opcode! {0x1C, "INC E", "E", "",   1, inc},
	make_opcode! {0x1D, "DEC E", "E", "",   1, dec},
	make_opcode! {0x1E, "LD E,d8", "E", "d",   2, ld},
	make_opcode! {0x1F, "RRA", "",  "",   1, rra},
	make_opcode! {0x20, "JR NZ,r8", "NZ", "d",   2, jr},
	make_opcode! {0x21, "LD HL,d16", "HL", "dd",   3, ld},
	make_opcode! {0x22, "LD (HL+),A", "(HLI)", "A",  2, ld},
	make_opcode! {0x23, "INC HL", "HL", "",   2, inc},
	make_opcode! {0x24, "INC H", "H", "",   1, inc},
	make_opcode! {0x25, "DEC H", "H", "",   1, dec},
	make_opcode! {0x26, "LD H,d8", "H", "d",   2, ld},
	make_opcode! {0x27, "DAA", "",  "",   1, daa},
	make_opcode! {0x28, "JR Z,r8", "Z", "d",   2, jr},
	make_opcode! {0x29, "ADD HL,HL", "HL", "HL",  2, addr16r16},
	make_opcode! {0x2A, "LD A,(HL+)", "A", "(HLI)",  2, ld},
	make_opcode! {0x2B, "DEC HL", "HL", "",   2, dec},
	make_opcode! {0x2C, "INC L", "L", "",   1, inc},
	make_opcode! {0x2D, "DEC L", "L", "",   1, dec},
	make_opcode! {0x2E, "LD L,d8", "L", "d",   2, ld},
	make_opcode! {0x2F, "CPL", "",  "",   1, cpl},
	make_opcode! {0x30, "JR NC,r8", "NC", "d",   2, jr},
	make_opcode! {0x31, "LD SP,d16", "SP", "dd",   3, ld},
	make_opcode! {0x32, "LD (HL-),A", "(HLD)", "A",  2, ld},
	make_opcode! {0x33, "INC SP", "SP", "",   2, inc},
	make_opcode! {0x34, "INC (HL)", "(HL)", "",   3, inc},
	make_opcode! {0x35, "DEC (HL)", "(HL)", "",   3, dec},
	make_opcode! {0x36, "LD (HL),d8", "(HL)", "d",   3, ld},
	make_opcode! {0x37, "SCF", "",  "",   1, scf},
	make_opcode! {0x38, "JR C,r8", "C", "d",   2, jr},
	make_opcode! {0x39, "ADD HL,SP", "HL", "SP",  2, addr16r16},
	make_opcode! {0x3A, "LD A,(HL-)", "A", "(HLD)",  2, ld},
	make_opcode! {0x3B, "DEC SP", "SP", "",   2, dec},
	make_opcode! {0x3C, "INC A", "A", "",   1, inc},
	make_opcode! {0x3D, "DEC A", "A", "",   1, dec},
	make_opcode! {0x3E, "LD A,d8", "A", "d",   2, ld},
	make_opcode! {0x3F, "CCF", "",  "",   1, ccf},
	make_opcode! {0x40, "LD B, B", "B", "B",  1, ld},
	make_opcode! {0x41, "LD B, C", "B", "C",  1, ld},
	make_opcode! {0x42, "LD B, D", "B", "D",  1, ld},
	make_opcode! {0x43, "LD B, E", "B", "E",  1, ld},
	make_opcode! {0x44, "LD B, H", "B", "H",  1, ld},
	make_opcode! {0x45, "LD B, L", "B", "L",  1, ld},
	make_opcode! {0x46, "LD B, (HL)", "B", "(HL)",  2, ld},
	make_opcode! {0x47, "LD B, A", "B", "A",  1, ld},
	make_opcode! {0x48, "LD C, B", "C", "B",  1, ld},
	make_opcode! {0x49, "LD C, C", "C", "C",  1, ld},
	make_opcode! {0x4A, "LD C, D", "C", "D",  1, ld},
	make_opcode! {0x4B, "LD C, E", "C", "E",  1, ld},
	make_opcode! {0x4C, "LD C, H", "C", "H",  1, ld},
	make_opcode! {0x4D, "LD C, L", "C", "L",  1, ld},
	make_opcode! {0x4E, "LD C, (HL)", "C", "(HL)",  2, ld},
	make_opcode! {0x4F, "LD C, A", "C", "A",  1, ld},
	make_opcode! {0x50, "LD D, B", "D", "B",  1, ld},
	make_opcode! {0x51, "LD D, C", "D", "C",  1, ld},
	make_opcode! {0x52, "LD D, D", "D", "D",  1, ld},
	make_opcode! {0x53, "LD D, E", "D", "E",  1, ld},
	make_opcode! {0x54, "LD D, H", "D", "H",  1, ld},
	make_opcode! {0x55, "LD D, L", "D", "L",  1, ld},
	make_opcode! {0x56, "LD D, (HL)", "D", "(HL)",  2, ld},
	make_opcode! {0x57, "LD D, A", "D", "A",  1, ld},
	make_opcode! {0x58, "LD E, B", "E", "B",  1, ld},
	make_opcode! {0x59, "LD E, C", "E", "C",  1, ld},
	make_opcode! {0x5A, "LD E, D", "E", "D",  1, ld},
	make_opcode! {0x5B, "LD E, E", "E", "E",  1, ld},
	make_opcode! {0x5C, "LD E, H", "E", "H",  1, ld},
	make_opcode! {0x5D, "LD E, L", "E", "L",  1, ld},
	make_opcode! {0x5E, "LD E, (HL)", "E", "(HL)",  2, ld},
	make_opcode! {0x5F, "LD E, A", "E", "A",  1, ld},
	make_opcode! {0x60, "LD H, B", "H", "B",  1, ld},
	make_opcode! {0x61, "LD H, C", "H", "C",  1, ld},
	make_opcode! {0x62, "LD H, D", "H", "D",  1, ld},
	make_opcode! {0x63, "LD H, E", "H", "E",  1, ld},
	make_opcode! {0x64, "LD H, H", "H", "H",  1, ld},
	make_opcode! {0x65, "LD H, L", "H", "L",  1, ld},
	make_opcode! {0x66, "LD H, (HL)", "H", "(HL)",  2, ld},
	make_opcode! {0x67, "LD H, A", "H", "A",  1, ld},
	make_opcode! {0x68, "LD L, B", "L", "B",  1, ld},
	make_opcode! {0x69, "LD L, C", "L", "C",  1, ld},
	make_opcode! {0x6A, "LD L, D", "L", "D",  1, ld},
	make_opcode! {0x6B, "LD L, E", "L", "E",  1, ld},
	make_opcode! {0x6C, "LD L, H", "L", "H",  1, ld},
	make_opcode! {0x6D, "LD L, L", "L", "L",  1, ld},
	make_opcode! {0x6E, "LD L, (HL)", "L", "(HL)",  2, ld},
	make_opcode! {0x6F, "LD L, A", "L", "A",  1, ld},
	make_opcode! {0x70, "LD (HL), B", "(HL)", "B",  2, ld},
	make_opcode! {0x71, "LD (HL), C", "(HL)", "C",  2, ld},
	make_opcode! {0x72, "LD (HL), D", "(HL)", "D",  2, ld},
	make_opcode! {0x73, "LD (HL), E", "(HL)", "E",  2, ld},
	make_opcode! {0x74, "LD (HL), H", "(HL)", "H",  2, ld},
	make_opcode! {0x75, "LD (HL), L", "(HL)", "L",  2, ld},
	make_opcode! {0x76, "HALT", "",  "",   1, halt},
	make_opcode! {0x77, "LD (HL), A", "(HL)", "A",  2, ld},
	make_opcode! {0x78, "LD A, B", "A", "B",  1, ld},
	make_opcode! {0x79, "LD A, C", "A", "C",  1, ld},
	make_opcode! {0x7A, "LD A, D", "A", "D",  1, ld},
	make_opcode! {0x7B, "LD A, E", "A", "E",  1, ld},
	make_opcode! {0x7C, "LD A, H", "A", "H",  1, ld},
	make_opcode! {0x7D, "LD A, L", "A", "L",  1, ld},
	make_opcode! {0x7E, "LD A, (HL)", "A", "(HL)",  2, ld},
	make_opcode! {0x7F, "LD A, A", "A", "A",  1, ld},
	make_opcode! {0x80, "ADD A, B", "A", "B",  1, add},
	make_opcode! {0x81, "ADD A, C", "A", "C",  1, add},
	make_opcode! {0x82, "ADD A, D", "A", "D",  1, add},
	make_opcode! {0x83, "ADD A, E", "A", "E",  1, add},
	make_opcode! {0x84, "ADD A, H", "A", "H",  1, add},
	make_opcode! {0x85, "ADD A, L", "A", "L",  1, add},
	make_opcode! {0x86, "ADD A, (HL)", "A", "(HL)",  2, add},
	make_opcode! {0x87, "ADD A, A", "A", "A",  1, add},
	make_opcode! {0x88, "ADC A, B", "A", "B",  1, adc},
	make_opcode! {0x89, "ADC A, C", "A", "C",  1, adc},
	make_opcode! {0x8A, "ADC A, D", "A", "D",  1, adc},
	make_opcode! {0x8B, "ADC A, E", "A", "E",  1, adc},
	make_opcode! {0x8C, "ADC A, H", "A", "H",  1, adc},
	make_opcode! {0x8D, "ADC A, L", "A", "L",  1, adc},
	make_opcode! {0x8E, "ADC A, (HL)", "A", "(HL)",  2, adc},
	make_opcode! {0x8F, "ADC A, A", "A", "A",  1, adc},
	make_opcode! {0x90, "SUB B", "A", "B", 1, sub},
	make_opcode! {0x91, "SUB C", "A", "C", 1, sub},
	make_opcode! {0x92, "SUB D", "A", "D", 1, sub},
	make_opcode! {0x93, "SUB E", "A", "E", 1, sub},
	make_opcode! {0x94, "SUB H", "A", "H", 1, sub},
	make_opcode! {0x95, "SUB L", "A", "L", 1, sub},
	make_opcode! {0x96, "SUB (HL)", "A", "(HL)", 2, sub},
	make_opcode! {0x97, "SUB A", "A", "A",   1, sub},
	make_opcode! {0x98, "SBC A, B", "A", "B",  1, sbc},
	make_opcode! {0x99, "SBC A, C", "A", "C",  1, sbc},
	make_opcode! {0x9A, "SBC A, D", "A", "D",  1, sbc},
	make_opcode! {0x9B, "SBC A, E", "A", "E",  1, sbc},
	make_opcode! {0x9C, "SBC A, H", "A", "H",  1, sbc},
	make_opcode! {0x9D, "SBC A, L", "A", "L",  1, sbc},
	make_opcode! {0x9E, "SBC A, (HL)", "A", "(HL)",  2, sbc},
	make_opcode! {0x9F, "SBC A, A", "A", "A",  1, sbc},
	make_opcode! {0xA0, "AND B", "B", "",   1, and},
	make_opcode! {0xA1, "AND C", "C", "",   1, and},
	make_opcode! {0xA2, "AND D", "D", "",   1, and},
	make_opcode! {0xA3, "AND E", "E", "",   1, and},
	make_opcode! {0xA4, "AND H", "H", "",   1, and},
	make_opcode! {0xA5, "AND L", "L", "",   1, and},
	make_opcode! {0xA6, "AND (HL)", "(HL)", "",   2, and},
	make_opcode! {0xA7, "AND A", "A", "",   1, and},
	make_opcode! {0xA8, "XOR B", "B", "",   1, xor},
	make_opcode! {0xA9, "XOR C", "C", "",   1, xor},
	make_opcode! {0xAA, "XOR D", "D", "",   1, xor},
	make_opcode! {0xAB, "XOR E", "E", "",   1, xor},
	make_opcode! {0xAC, "XOR H", "H", "",   1, xor},
	make_opcode! {0xAD, "XOR L", "L", "",   1, xor},
	make_opcode! {0xAE, "XOR (HL)", "(HL)", "",   2, xor},
	make_opcode! {0xAF, "XOR A", "A", "",   1, xor},
	make_opcode! {0xB0, "OR B", "B", "",   1, or},
	make_opcode! {0xB1, "OR C", "C", "",   1, or},
	make_opcode! {0xB2, "OR D", "D", "",   1, or},
	make_opcode! {0xB3, "OR E", "E", "",   1, or},
	make_opcode! {0xB4, "OR H", "H", "",   1, or},
	make_opcode! {0xB5, "OR L", "L", "",   1, or},
	make_opcode! {0xB6, "OR (HL)", "(HL)", "",   2, or},
	make_opcode! {0xB7, "OR A", "A", "",   1, or},
	make_opcode! {0xB8, "CP B", "B", "",   1, cp},
	make_opcode! {0xB9, "CP C", "C", "",   1, cp},
	make_opcode! {0xBA, "CP D", "D", "",   1, cp},
	make_opcode! {0xBB, "CP E", "E", "",   1, cp},
	make_opcode! {0xBC, "CP H", "H", "",   1, cp},
	make_opcode! {0xBD, "CP L", "L", "",   1, cp},
	make_opcode! {0xBE, "CP (HL)", "(HL)", "",   2, cp},
	make_opcode! {0xBF, "CP A", "A", "",   1, cp},
	make_opcode! {0xC0, "RET NZ", "NZ", "",   2, ret},
	make_opcode! {0xC1, "POP BC", "BC", "",   3, pop},
	make_opcode! {0xC2, "JP NZ,a16", "NZ", "aa",   3, jp},
	make_opcode! {0xC3, "JP a16", "aa",  "",   4, jp},
	make_opcode! {0xC4, "CALL NZ,a16", "NZ", "aa",   3, call},
	make_opcode! {0xC5, "PUSH BC", "BC", "",   4, push},
	make_opcode! {0xC6, "ADD A,d8", "A", "d",   2, add},
	make_opcode! {0xC7, "RST 00H", "00", "",   4, rst},
	make_opcode! {0xC8, "RET Z", "Z", "",   2, ret},
	make_opcode! {0xC9, "RET", "",  "",   4, ret},
	make_opcode! {0xCA, "JP Z,a16", "Z", "aa",   3, jp},
	make_opcode! {0xCB, "PREFIX CB", "",  "",   1, empty},
	make_opcode! {0xCC, "CALL Z,a16", "Z", "aa",   3, call},
	make_opcode! {0xCD, "CALL a16", "aa",  "",   6, call},
	make_opcode! {0xCE, "ADC A,d8", "A", "d",   2, adc},
	make_opcode! {0xCF, "RST 08H", "08", "",   4, rst},
	make_opcode! {0xD0, "RET NC", "NC", "",   2, ret},
	make_opcode! {0xD1, "POP DE", "DE", "",   3, pop},
	make_opcode! {0xD2, "JP NC,a16", "NC", "aa",   3, jp},
	make_opcode! {0xD3, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xD4, "CALL NC,a16", "NC", "aa",   3, call},
	make_opcode! {0xD5, "PUSH DE", "DE", "",   4, push},
	make_opcode! {0xD6, "SUB d8", "A",  "d",   2, sub},
	make_opcode! {0xD7, "RST 10H", "16", "",   4, rst},
	make_opcode! {0xD8, "RET C", "C", "",   2, ret},
	make_opcode! {0xD9, "RETI", "",  "",   4, reti},
	make_opcode! {0xDA, "JP C,a16", "C", "aa",   3, jp},
	make_opcode! {0xDB, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xDC, "CALL C,a16", "C", "aa",   3, call},
	make_opcode! {0xDD, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xDE, "SBC A,d8", "A", "d",   2, sbc},
	make_opcode! {0xDF, "RST 18H", "24", "",   4, rst},
	make_opcode! {0xE0, "LDH (a8),A", "(a)",  "A",  3, ld},
	make_opcode! {0xE1, "POP HL", "HL", "",   3, pop},
	make_opcode! {0xE2, "LD (C),A", "(C)", "A",  2, ld},
	make_opcode! {0xE3, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xE4, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xE5, "PUSH HL", "HL", "",   4, push},
	make_opcode! {0xE6, "AND d8", "d",  "",   2, and},
	make_opcode! {0xE7, "RST 20H", "32", "",   4, rst},
	make_opcode! {0xE8, "ADD SP,r8", "SP", "d",   4, addr16d},
	make_opcode! {0xE9, "JP (HL)", "HL", "",   1, jp},
	make_opcode! {0xEA, "LD (a16),A", "(aa)",  "A",  4, ld},
	make_opcode! {0xEB, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xEC, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xED, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xEE, "XOR d8", "d",  "",   2, xor},
	make_opcode! {0xEF, "RST 28H", "40", "",   4, rst},
	make_opcode! {0xF0, "LDH A,(a8)", "A", "(a)",   3, ld},
	make_opcode! {0xF1, "POP AF", "AF",  "",   3, pop},
	make_opcode! {0xF2, "LD A,(C)", "A", "(C)",  2, ld},
	make_opcode! {0xF3, "DI", "",  "",   1, di},
	make_opcode! {0xF4, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xF5, "PUSH AF", "AF",  "",   4, push},
	make_opcode! {0xF6, "OR d8", "d",  "",   2, or},
	make_opcode! {0xF7, "RST 30H", "48", "",   4, rst},
	make_opcode! {0xF8, "LD HL,SP+r8", "HL", "SP",  3, ldr16r16d},
	make_opcode! {0xF9, "LD SP,HL", "SP", "HL",  2, ld},
	make_opcode! {0xFA, "LD A,(a16)", "A", "(aa)",   4, ld},
	make_opcode! {0xFB, "EI", "",  "",   1, ei},
	make_opcode! {0xFC, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xFD, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xFE, "CP d8", "d",  "",   2, cp},
	make_opcode! {0xFF, "RST 38H", "56", "",   4, rst},
]);

fn empty(_: &mut Cpu, _: String, _: String) {
	unreachable!("this is empty!");
}

fn nop(_: &mut Cpu, _: String, _: String) {
}

fn stop(_: &mut Cpu, _: String, _: String) {
	println!("stop impl");
}

fn ld(c: &mut Cpu, r1: String, r2: String) {
	let value = c.load(&r2);
	c.store(&r1, value);
}

// LD r1, r2+r
fn ldr16r16d(c: &mut Cpu, r1: String, r2: String) {
	let r = c.fetch() as i8 as Word;
	let sp = c.load(&r2);
	let v = sp.wrapping_add(r);
	c.store(&r1, v);

	c.reg.F.z = false;
	c.reg.F.n = false;
	c.reg.F.h = (r ^ sp ^ v) & 0x10 != 0;
	c.reg.F.c = (r ^ sp ^ v) & 0x100 != 0;
}

// arithmetic
fn inc(c: &mut Cpu, r1: String, _: String) {
	let r = c.load(&r1);
	let value = r.wrapping_add(0x01);

	if R_ARR.contains(&r1.as_str()) || MM_ARR.contains(&r1.as_str()) {
		c.reg.F.z = value as Byte == 0;
		c.reg.F.n = false;
		c.reg.F.h = (r as Byte ^ value as Byte) & 0x10 != 0;
	}
	c.store(&r1, value);
}

fn dec(c: &mut Cpu, r1: String, _: String) {
	let r = c.load(&r1);
	let value = r.wrapping_sub(0x01);

	if R_ARR.contains(&r1.as_str()) || MM_ARR.contains(&r1.as_str()) {
		c.reg.F.z = value == 0;
		c.reg.F.n = true;
		c.reg.F.h = (r ^ value) & 0x10 != 0;
	}
	c.store(&r1, value);
}

fn and(c: &mut Cpu, r1: String, _: String) {
	let r = c.load(&r1);
	let a = c.load(&"A".to_string());
	let value = a & r;
	c.store(&"A".to_string(), value as Word);

	c.reg.F.z = value == 0;
	c.reg.F.n = false;
	c.reg.F.h = true;
	c.reg.F.c = false;
}

fn or(c: &mut Cpu, r1: String, _: String) {
	let r = c.load(&r1);
	let a = c.load(&"A".to_string());
	let value = a | r;
	c.store(&"A".to_string(), value as Word);

	c.reg.F.z = value == 0;
	c.reg.F.n = false;
	c.reg.F.h = false;
	c.reg.F.c = false;
}

fn xor(c: &mut Cpu, r1: String, _: String) {
	let r = c.load(&r1);
	let a = c.load(&"A".to_string());
	let value = a ^ r;
	c.store(&"A".to_string(), value as Word);

	c.reg.F.z = value == 0;
	c.reg.F.n = false;
	c.reg.F.h = false;
	c.reg.F.c = false;
}

fn cp(c: &mut Cpu, r1: String, _: String) {
	let r = c.load(&r1);
	let a = c.load(&"A".to_string());

	let (res, overflow) = a.overflowing_sub(r);

	c.reg.F.z = a == r;
	c.reg.F.n = true;
	c.reg.F.h = (a ^ r ^ res) & 0x10 != 0;
	c.reg.F.c = overflow;
}

fn add(c: &mut Cpu, r1: String, r2: String) {
	let a = c.load(&r1) as Byte;
	let r = c.load(&r2) as Byte;

	let (v, overflow) = a.overflowing_add(r);

	c.reg.F.z = v == 0;
	c.reg.F.n = false;
	c.reg.F.h = (a ^ r ^ v) & 0x10 != 0;
	c.reg.F.c = overflow;

	c.store(&r1, v as Word);
}

fn addr16r16(c: &mut Cpu, r1: String, r2: String) {
	let a = c.load(&r1);
	let b = c.load(&r2);

	let (v, overflow) = a.overflowing_add(b);

	c.reg.F.n = false;
	c.reg.F.h = (a ^ b ^ v) & 0x1000 != 0;
	c.reg.F.c = overflow;

	c.store(&r1, v);
}

fn addr16d(c: &mut Cpu, r1: String, r2: String) {
	let v1 = c.load(&r1);
	let v2 = c.load(&r2) as i8 as Word;

	let v = v1.wrapping_add(v2);

	c.reg.F.z = false;
	c.reg.F.n = false;
	c.reg.F.h = (v1 ^ v2 ^ v) & 0x10 != 0;
	c.reg.F.c = (v1 ^ v2 ^ v) & 0x100 != 0;

	c.store(&r1, v);
}

fn adc(c: &mut Cpu, r1: String, r2: String) {
	let a = c.load(&r1) as Byte;
	let r = c.load(&r2) as Byte;
	let carry = if c.reg.F.c {1} else {0};

	let (v, overflow1) = a.overflowing_add(r);
	let (v, overflow2) = v.overflowing_add(carry);

	c.reg.F.z = v == 0;
	c.reg.F.n = false;
	c.reg.F.h = (a ^ r ^ v) & 0x10 != 0;
	c.reg.F.c = overflow1 | overflow2;

	c.store(&r1, v as Word);
}

fn sub(c: &mut Cpu, r1: String, r2: String) {
	let a = c.load(&r1) as Byte;
	let r = c.load(&r2) as Byte;

	let (v, overflow) = a.overflowing_sub(r);

	c.reg.F.z = v == 0;
	c.reg.F.n = true;
	c.reg.F.h = (a ^ r ^ v) & 0x10 != 0;
	c.reg.F.c = overflow;

	c.store(&r1, v as Word);
}

fn sbc(c: &mut Cpu, r1: String, r2: String) {
	let a = c.load(&r1) as Byte;
	let r = c.load(&r2) as Byte;
	let carry = if c.reg.F.c {1} else {0};

	let (v, overflow1) = a.overflowing_sub(r);
	let (v, overflow2) = v.overflowing_sub(carry);

	c.reg.F.z = v == 0;
	c.reg.F.n = true;
	c.reg.F.h = (a ^ r ^ v) & 0x10 != 0;
	c.reg.F.c = overflow1 | overflow2;

	c.store(&r1, v as Word);
}

// jp
fn jp(c: &mut Cpu, r1: String, r2: String) {
	let addr: Word;
	if COND_ARR.contains(&r1.as_str()) {
		addr = c.load(&r2);
		if c.cond(&r1) {
			c.reg.PC = addr
		}
	} else {
		addr = c.load(&r1);
		c.reg.PC = addr
	}
}

fn jr(c: &mut Cpu, r1: String, r2: String) {
	let addr: Word;
	if COND_ARR.contains(&r1.as_str()) {
		addr = c.load(&r2);
		if c.cond(&r1) {
			c.reg.PC = ((c.reg.PC as i32) + (addr as i8) as i32) as Word;
		}
	} else {
		addr = c.load(&r1);
		c.reg.PC = ((c.reg.PC as i32) + (addr as i8) as i32) as Word;
	}
}

fn call(c: &mut Cpu, r1: String, r2: String) {
	let addr: Word;
	if COND_ARR.contains(&r1.as_str()) {
		addr = c.load(&r2);
		if c.cond(&r1) {
			c.push_pc();
			c.reg.PC = addr;
		}
	} else {
		addr = c.load(&r1);
		c.push_pc();
		c.reg.PC = addr;
	}
}

// ret
fn ret(c: &mut Cpu, r1: String, _: String) {
	if COND_ARR.contains(&r1.as_str()) {
		if c.cond(&r1) {
			c.pop_pc()
		}
	} else {
		c.pop_pc()
	}
}

fn reti(c: &mut Cpu, _: String, _: String) {
	c.pop_pc();
	c.ime = true;
}

// -----rst------

// RST n
// push and jump to n
fn rst(c: &mut Cpu, n: String, _: String) {
	c.push_pc();
	let num: Word = n.parse().unwrap();
	c.reg.PC = num;
}

// -----push-----
fn push(c: &mut Cpu, r: String, _: String) {
	let buf = c.load(&r);
	let upper = extract_upper(buf);
	let lower = extract_lower(buf);
	c.push(upper);
	c.push(lower);
}

// -----pop------
fn pop(c: &mut Cpu, r: String, _: String) {
	let mut lower = c.pop();
	let upper = c.pop();

	if r == "AF" {
		lower &= 0xF0;
	}

	let value = bytes_2_word(upper, lower);
	c.store(&r, value);
}

// -----misc-----
fn rra(c: &mut Cpu, _: String, _: String) {
	let r = c.load(&"A".to_string());
	let carry = c.reg.F.c;
	let mut value = r >> 1;

	if carry {
		value |= 0x80;
	}

	c.reg.F.z = false;
	c.reg.F.n = false;
	c.reg.F.h = false;
	c.reg.F.c = r & 0x01 == 0x01;

	c.store(&"A".to_string(), value);
}

fn rla(c: &mut Cpu, _: String, _: String) {
	let r = c.load(&"A".to_string()) as Byte;
	let carry = c.reg.F.c;
	let mut value = r << 1;

	if carry {
		value += 1;
	}

	c.reg.F.z = false;
	c.reg.F.n = false;
	c.reg.F.h = false;
	c.reg.F.c = r & 0x80 == 0x80;

	c.store(&"A".to_string(), value as Word);
}

fn rlca(c: &mut Cpu, _: String, _: String) {
	let r = c.load(&"A".to_string()) as Byte;
	let value = r.rotate_left(1);

	c.reg.F.z = false;
	c.reg.F.n = false;
	c.reg.F.h = false;
	c.reg.F.c = (r & 0x80) == 0x80;

	c.store(&"A".to_string(), value as Word);
}

fn rrca(c: &mut Cpu, _: String, _: String) {
	let r = c.load(&"A".to_string()) as Byte;
	let value = r.rotate_right(1);

	c.reg.F.z = false;
	c.reg.F.n = false;
	c.reg.F.h = false;
	c.reg.F.c = r & 0x01 == 0x01;

	c.store(&"A".to_string(), value as Word);
}

// @see https://donkeyhacks.zouri.jp/html/En-Us/snes/apu/spc700/daa.html
fn daa(c: &mut Cpu, _: String, _: String) {
	let mut a = c.load(&"A".to_string());

	if !c.reg.F.n {
		if a & 0x0F >= 0x0A || c.reg.F.h {
			a = a.wrapping_add(0x06);
		}
		if a >= 0xA0 || c.reg.F.c {
			a = a.wrapping_add(0x60);
		}
	} else {
		if c.reg.F.h {
			a = a.wrapping_sub(0x06) & 0xFF;
		}
		if c.reg.F.c {
			a = a.wrapping_sub(0x60);
		}
	}

	c.store(&"A".to_string(), a as Byte as Word);
	c.reg.F.z = a as Byte == 0;
	c.reg.F.h = false;
	if a&0x100 == 0x100 {
		c.reg.F.c = true;
	}
}


fn cpl(c: &mut Cpu, _: String, _: String) {
	let a = c.load(&"A".to_string()) as Byte;
	c.store(&"A".to_string(), (a ^ 0xFF) as Word);

	c.reg.F.n = true;
	c.reg.F.h = true;
}

fn scf(c: &mut Cpu, _: String, _: String) {
	c.reg.F.n = false;
	c.reg.F.h = false;
	c.reg.F.c = true;
}

fn ccf(c: &mut Cpu, _: String, _: String) {
	c.reg.F.n = false;
	c.reg.F.h = false;
	c.reg.F.c = !c.reg.F.c;
}

fn di(c: &mut Cpu, _: String, _: String) {
	c.ime = false;
}

fn ei(c: &mut Cpu, _: String, _: String) {
	c.ime = true;
}

fn halt(c: &mut Cpu, _: String, _: String) {
	c.halted = true;
}

#[rustfmt::skip]
pub static CB_OPCODES: Lazy<[OpCode; 256]> = Lazy::new(|| [
	make_opcode! {0x00, "RLC B", "B", "", 2, rlc},
	make_opcode! {0x01, "RLC C", "C", "", 2, rlc},
	make_opcode! {0x02, "RLC D", "D", "", 2, rlc},
	make_opcode! {0x03, "RLC E", "E", "", 2, rlc},
	make_opcode! {0x04, "RLC H", "H", "", 2, rlc},
	make_opcode! {0x05, "RLC L", "L", "", 2, rlc},
	make_opcode! {0x06, "RLC (HL)", "(HL)", "", 4, rlc},
	make_opcode! {0x07, "RLC A", "A", "", 2, rlc},
	make_opcode! {0x08, "RRC B", "B", "", 2, rrc},
	make_opcode! {0x09, "RRC C", "C", "", 2, rrc},
	make_opcode! {0x0A, "RRC D", "D", "", 2, rrc},
	make_opcode! {0x0B, "RRC E", "E", "", 2, rrc},
	make_opcode! {0x0C, "RRC H", "H", "", 2, rrc},
	make_opcode! {0x0D, "RRC L", "L", "", 2, rrc},
	make_opcode! {0x0E, "RRC (HL)", "(HL)", "", 4, rrc},
	make_opcode! {0x0F, "RRC A", "A", "", 2, rrc},
	make_opcode! {0x10, "RL B", "B", "", 2, rl},
	make_opcode! {0x11, "RL C", "C", "", 2, rl},
	make_opcode! {0x12, "RL D", "D", "", 2, rl},
	make_opcode! {0x13, "RL E", "E", "", 2, rl},
	make_opcode! {0x14, "RL H", "H", "", 2, rl},
	make_opcode! {0x15, "RL L", "L", "", 2, rl},
	make_opcode! {0x16, "RL (HL)", "(HL)", "", 4, rl},
	make_opcode! {0x17, "RL A", "A", "", 2, rl},
	make_opcode! {0x18, "RR B", "B", "", 2, rr},
	make_opcode! {0x19, "RR C", "C", "", 2, rr},
	make_opcode! {0x1A, "RR D", "D", "", 2, rr},
	make_opcode! {0x1B, "RR E", "E", "", 2, rr},
	make_opcode! {0x1C, "RR H", "H", "", 2, rr},
	make_opcode! {0x1D, "RR L", "L", "", 2, rr},
	make_opcode! {0x1E, "RR (HL)", "(HL)", "", 4, rr},
	make_opcode! {0x1F, "RR A", "A", "", 2, rr},
	make_opcode! {0x20, "SLA B", "B", "", 2, sla},
	make_opcode! {0x21, "SLA C", "C", "", 2, sla},
	make_opcode! {0x22, "SLA D", "D", "", 2, sla},
	make_opcode! {0x23, "SLA E", "E", "", 2, sla},
	make_opcode! {0x24, "SLA H", "H", "", 2, sla},
	make_opcode! {0x25, "SLA L", "L", "", 2, sla},
	make_opcode! {0x26, "SLA (HL)", "(HL)", "", 4, sla},
	make_opcode! {0x27, "SLA A", "A", "", 2, sla},
	make_opcode! {0x28, "SRA B", "B", "", 2, sra},
	make_opcode! {0x29, "SRA C", "C", "", 2, sra},
	make_opcode! {0x2A, "SRA D", "D", "", 2, sra},
	make_opcode! {0x2B, "SRA E", "E", "", 2, sra},
	make_opcode! {0x2C, "SRA H", "H", "", 2, sra},
	make_opcode! {0x2D, "SRA L", "L", "", 2, sra},
	make_opcode! {0x2E, "SRA (HL)", "(HL)", "", 4, sra},
	make_opcode! {0x2F, "SRA A", "A", "", 2, sra},
	make_opcode! {0x30, "SWAP B", "B", "", 2, swap},
	make_opcode! {0x31, "SWAP C", "C", "", 2, swap},
	make_opcode! {0x32, "SWAP D", "D", "", 2, swap},
	make_opcode! {0x33, "SWAP E", "E", "", 2, swap},
	make_opcode! {0x34, "SWAP H", "H", "", 2, swap},
	make_opcode! {0x35, "SWAP L", "L", "", 2, swap},
	make_opcode! {0x36, "SWAP (HL)", "(HL)", "", 4, swap},
	make_opcode! {0x37, "SWAP A", "A", "", 2, swap},
	make_opcode! {0x38, "SRL B", "B", "", 2, srl},
	make_opcode! {0x39, "SRL C", "C", "", 2, srl},
	make_opcode! {0x3A, "SRL D", "D", "", 2, srl},
	make_opcode! {0x3B, "SRL E", "E", "", 2, srl},
	make_opcode! {0x3C, "SRL H", "H", "", 2, srl},
	make_opcode! {0x3D, "SRL L", "L", "", 2, srl},
	make_opcode! {0x3E, "SRL (HL)", "(HL)", "", 4, srl},
	make_opcode! {0x3F, "SRL A", "A", "", 2, srl},
	make_opcode! {0x40, "BIT 0,B", "0", "B", 2, bit},
	make_opcode! {0x41, "BIT 0,C", "0", "C", 2, bit},
	make_opcode! {0x42, "BIT 0,D", "0", "D", 2, bit},
	make_opcode! {0x43, "BIT 0,E", "0", "E", 2, bit},
	make_opcode! {0x44, "BIT 0,H", "0", "H", 2, bit},
	make_opcode! {0x45, "BIT 0,L", "0", "L", 2, bit},
	make_opcode! {0x46, "BIT 0,(HL)", "0", "(HL)", 3, bit},
	make_opcode! {0x47, "BIT 0,A", "0", "A", 2, bit},
	make_opcode! {0x48, "BIT 1,B", "1", "B", 2, bit},
	make_opcode! {0x49, "BIT 1,C", "1", "C", 2, bit},
	make_opcode! {0x4A, "BIT 1,D", "1", "D", 2, bit},
	make_opcode! {0x4B, "BIT 1,E", "1", "E", 2, bit},
	make_opcode! {0x4C, "BIT 1,H", "1", "H", 2, bit},
	make_opcode! {0x4D, "BIT 1,L", "1", "L", 2, bit},
	make_opcode! {0x4E, "BIT 1,(HL)", "1", "(HL)", 3, bit},
	make_opcode! {0x4F, "BIT 1,A", "1", "A", 2, bit},
	make_opcode! {0x50, "BIT 2,B", "2", "B", 2, bit},
	make_opcode! {0x51, "BIT 2,C", "2", "C", 2, bit},
	make_opcode! {0x52, "BIT 2,D", "2", "D", 2, bit},
	make_opcode! {0x53, "BIT 2,E", "2", "E", 2, bit},
	make_opcode! {0x54, "BIT 2,H", "2", "H", 2, bit},
	make_opcode! {0x55, "BIT 2,L", "2", "L", 2, bit},
	make_opcode! {0x56, "BIT 2,(HL)", "2", "(HL)", 3, bit},
	make_opcode! {0x57, "BIT 2,A", "2", "A", 2, bit},
	make_opcode! {0x58, "BIT 3,B", "3", "B", 2, bit},
	make_opcode! {0x59, "BIT 3,C", "3", "C", 2, bit},
	make_opcode! {0x5A, "BIT 3,D", "3", "D", 2, bit},
	make_opcode! {0x5B, "BIT 3,E", "3", "E", 2, bit},
	make_opcode! {0x5C, "BIT 3,H", "3", "H", 2, bit},
	make_opcode! {0x5D, "BIT 3,L", "3", "L", 2, bit},
	make_opcode! {0x5E, "BIT 3,(HL)", "3", "(HL)", 3, bit},
	make_opcode! {0x5F, "BIT 3,A", "3", "A", 2, bit},
	make_opcode! {0x60, "BIT 4,B", "4", "B", 2, bit},
	make_opcode! {0x61, "BIT 4,C", "4", "C", 2, bit},
	make_opcode! {0x62, "BIT 4,D", "4", "D", 2, bit},
	make_opcode! {0x63, "BIT 4,E", "4", "E", 2, bit},
	make_opcode! {0x64, "BIT 4,H", "4", "H", 2, bit},
	make_opcode! {0x65, "BIT 4,L", "4", "L", 2, bit},
	make_opcode! {0x66, "BIT 4,(HL)", "4", "(HL)", 3, bit},
	make_opcode! {0x67, "BIT 4,A", "4", "A", 2, bit},
	make_opcode! {0x68, "BIT 5,B", "5", "B", 2, bit},
	make_opcode! {0x69, "BIT 5,C", "5", "C", 2, bit},
	make_opcode! {0x6A, "BIT 5,D", "5", "D", 2, bit},
	make_opcode! {0x6B, "BIT 5,E", "5", "E", 2, bit},
	make_opcode! {0x6C, "BIT 5,H", "5", "H", 2, bit},
	make_opcode! {0x6D, "BIT 5,L", "5", "L", 2, bit},
	make_opcode! {0x6E, "BIT 5,(HL)", "5", "(HL)", 3, bit},
	make_opcode! {0x6F, "BIT 5,A", "5", "A", 2, bit},
	make_opcode! {0x70, "BIT 6,B", "6", "B", 2, bit},
	make_opcode! {0x71, "BIT 6,C", "6", "C", 2, bit},
	make_opcode! {0x72, "BIT 6,D", "6", "D", 2, bit},
	make_opcode! {0x73, "BIT 6,E", "6", "E", 2, bit},
	make_opcode! {0x74, "BIT 6,H", "6", "H", 2, bit},
	make_opcode! {0x75, "BIT 6,L", "6", "L", 2, bit},
	make_opcode! {0x76, "BIT 6,(HL)", "6", "(HL)", 3, bit},
	make_opcode! {0x77, "BIT 6,A", "6", "A", 2, bit},
	make_opcode! {0x78, "BIT 7,B", "7", "B", 2, bit},
	make_opcode! {0x79, "BIT 7,C", "7", "C", 2, bit},
	make_opcode! {0x7A, "BIT 7,D", "7", "D", 2, bit},
	make_opcode! {0x7B, "BIT 7,E", "7", "E", 2, bit},
	make_opcode! {0x7C, "BIT 7,H", "7", "H", 2, bit},
	make_opcode! {0x7D, "BIT 7,L", "7", "L", 2, bit},
	make_opcode! {0x7E, "BIT 7,(HL)", "7", "(HL)", 3, bit},
	make_opcode! {0x7F, "BIT 7,A", "7", "A", 2, bit},
	make_opcode! {0x80, "RES 0,B", "0", "B", 2, res},
	make_opcode! {0x81, "RES 0,C", "0", "C", 2, res},
	make_opcode! {0x82, "RES 0,D", "0", "D", 2, res},
	make_opcode! {0x83, "RES 0,E", "0", "E", 2, res},
	make_opcode! {0x84, "RES 0,H", "0", "H", 2, res},
	make_opcode! {0x85, "RES 0,L", "0", "L", 2, res},
	make_opcode! {0x86, "RES 0,(HL)", "0", "(HL)",  4, res},
	make_opcode! {0x87, "RES 0,A", "0", "A", 2, res},
	make_opcode! {0x88, "RES 1,B", "1", "B", 2, res},
	make_opcode! {0x89, "RES 1,C", "1", "C", 2, res},
	make_opcode! {0x8A, "RES 1,D", "1", "D", 2, res},
	make_opcode! {0x8B, "RES 1,E", "1", "E", 2, res},
	make_opcode! {0x8C, "RES 1,H", "1", "H", 2, res},
	make_opcode! {0x8D, "RES 1,L", "1", "L", 2, res},
	make_opcode! {0x8E, "RES 1,(HL)", "1", "(HL)",  4, res},
	make_opcode! {0x8F, "RES 1,A", "1", "A", 2, res},
	make_opcode! {0x90, "RES 2,B", "2", "B", 2, res},
	make_opcode! {0x91, "RES 2,C", "2", "C", 2, res},
	make_opcode! {0x92, "RES 2,D", "2", "D", 2, res},
	make_opcode! {0x93, "RES 2,E", "2", "E", 2, res},
	make_opcode! {0x94, "RES 2,H", "2", "H", 2, res},
	make_opcode! {0x95, "RES 2,L", "2", "L", 2, res},
	make_opcode! {0x96, "RES 2,(HL)", "2", "(HL)",  4, res},
	make_opcode! {0x97, "RES 2,A", "2", "A", 2, res},
	make_opcode! {0x98, "RES 3,B", "3", "B", 2, res},
	make_opcode! {0x99, "RES 3,C", "3", "C", 2, res},
	make_opcode! {0x9A, "RES 3,D", "3", "D", 2, res},
	make_opcode! {0x9B, "RES 3,E", "3", "E", 2, res},
	make_opcode! {0x9C, "RES 3,H", "3", "H", 2, res},
	make_opcode! {0x9D, "RES 3,L", "3", "L", 2, res},
	make_opcode! {0x9E, "RES 3,(HL)", "3", "(HL)",  4, res},
	make_opcode! {0x9F, "RES 3,A", "3", "A", 2, res},
	make_opcode! {0xA0, "RES 4,B", "4", "B", 2, res},
	make_opcode! {0xA1, "RES 4,C", "4", "C", 2, res},
	make_opcode! {0xA2, "RES 4,D", "4", "D", 2, res},
	make_opcode! {0xA3, "RES 4,E", "4", "E", 2, res},
	make_opcode! {0xA4, "RES 4,H", "4", "H", 2, res},
	make_opcode! {0xA5, "RES 4,L", "4", "L", 2, res},
	make_opcode! {0xA6, "RES 4,(HL)", "4", "(HL)",  4, res},
	make_opcode! {0xA7, "RES 4,A", "4", "A", 2, res},
	make_opcode! {0xA8, "RES 5,B", "5", "B", 2, res},
	make_opcode! {0xA9, "RES 5,C", "5", "C", 2, res},
	make_opcode! {0xAA, "RES 5,D", "5", "D", 2, res},
	make_opcode! {0xAB, "RES 5,E", "5", "E", 2, res},
	make_opcode! {0xAC, "RES 5,H", "5", "H", 2, res},
	make_opcode! {0xAD, "RES 5,L", "5", "L", 2, res},
	make_opcode! {0xAE, "RES 5,(HL)", "5", "(HL)",  4, res},
	make_opcode! {0xAF, "RES 5,A", "5", "A", 2, res},
	make_opcode! {0xB0, "RES 6,B", "6", "B", 2, res},
	make_opcode! {0xB1, "RES 6,C", "6", "C", 2, res},
	make_opcode! {0xB2, "RES 6,D", "6", "D", 2, res},
	make_opcode! {0xB3, "RES 6,E", "6", "E", 2, res},
	make_opcode! {0xB4, "RES 6,H", "6", "H", 2, res},
	make_opcode! {0xB5, "RES 6,L", "6", "L", 2, res},
	make_opcode! {0xB6, "RES 6,(HL)", "6", "(HL)",  4, res},
	make_opcode! {0xB7, "RES 6,A", "6", "A", 2, res},
	make_opcode! {0xB8, "RES 7,B", "7", "B", 2, res},
	make_opcode! {0xB9, "RES 7,C", "7", "C", 2, res},
	make_opcode! {0xBA, "RES 7,D", "7", "D", 2, res},
	make_opcode! {0xBB, "RES 7,E", "7", "E", 2, res},
	make_opcode! {0xBC, "RES 7,H", "7", "H", 2, res},
	make_opcode! {0xBD, "RES 7,L", "7", "L", 2, res},
	make_opcode! {0xBE, "RES 7,(HL)", "7", "(HL)",  4, res},
	make_opcode! {0xBF, "RES 7,A", "7", "A", 2, res},
	make_opcode! {0xC0, "SET 0,B", "0", "B", 2, set},
	make_opcode! {0xC1, "SET 0,C", "0", "C", 2, set},
	make_opcode! {0xC2, "SET 0,D", "0", "D", 2, set},
	make_opcode! {0xC3, "SET 0,E", "0", "E", 2, set},
	make_opcode! {0xC4, "SET 0,H", "0", "H", 2, set},
	make_opcode! {0xC5, "SET 0,L", "0", "L", 2, set},
	make_opcode! {0xC6, "SET 0,(HL)", "0", "(HL)",  4, set},
	make_opcode! {0xC7, "SET 0,A", "0", "A", 2, set},
	make_opcode! {0xC8, "SET 1,B", "1", "B", 2, set},
	make_opcode! {0xC9, "SET 1,C", "1", "C", 2, set},
	make_opcode! {0xCA, "SET 1,D", "1", "D", 2, set},
	make_opcode! {0xCB, "SET 1,E", "1", "E", 2, set},
	make_opcode! {0xCC, "SET 1,H", "1", "H", 2, set},
	make_opcode! {0xCD, "SET 1,L", "1", "L", 2, set},
	make_opcode! {0xCE, "SET 1,(HL)", "1", "(HL)",  4, set},
	make_opcode! {0xCF, "SET 1,A", "1", "A", 2, set},
	make_opcode! {0xD0, "SET 2,B", "2", "B", 2, set},
	make_opcode! {0xD1, "SET 2,C", "2", "C", 2, set},
	make_opcode! {0xD2, "SET 2,D", "2", "D", 2, set},
	make_opcode! {0xD3, "SET 2,E", "2", "E", 2, set},
	make_opcode! {0xD4, "SET 2,H", "2", "H", 2, set},
	make_opcode! {0xD5, "SET 2,L", "2", "L", 2, set},
	make_opcode! {0xD6, "SET 2,(HL)", "2", "(HL)",  4, set},
	make_opcode! {0xD7, "SET 2,A", "2", "A", 2, set},
	make_opcode! {0xD8, "SET 3,B", "3", "B", 2, set},
	make_opcode! {0xD9, "SET 3,C", "3", "C", 2, set},
	make_opcode! {0xDA, "SET 3,D", "3", "D", 2, set},
	make_opcode! {0xDB, "SET 3,E", "3", "E", 2, set},
	make_opcode! {0xDC, "SET 3,H", "3", "H", 2, set},
	make_opcode! {0xDD, "SET 3,L", "3", "L", 2, set},
	make_opcode! {0xDE, "SET 3,(HL)", "3", "(HL)",  4, set},
	make_opcode! {0xDF, "SET 3,A", "3", "A", 2, set},
	make_opcode! {0xE0, "SET 4,B", "4", "B", 2, set},
	make_opcode! {0xE1, "SET 4,C", "4", "C", 2, set},
	make_opcode! {0xE2, "SET 4,D", "4", "D", 2, set},
	make_opcode! {0xE3, "SET 4,E", "4", "E", 2, set},
	make_opcode! {0xE4, "SET 4,H", "4", "H", 2, set},
	make_opcode! {0xE5, "SET 4,L", "4", "L", 2, set},
	make_opcode! {0xE6, "SET 4,(HL)", "4", "(HL)",  4, set},
	make_opcode! {0xE7, "SET 4,A", "4", "A", 2, set},
	make_opcode! {0xE8, "SET 5,B", "5", "B", 2, set},
	make_opcode! {0xE9, "SET 5,C", "5", "C", 2, set},
	make_opcode! {0xEA, "SET 5,D", "5", "D", 2, set},
	make_opcode! {0xEB, "SET 5,E", "5", "E", 2, set},
	make_opcode! {0xEC, "SET 5,H", "5", "H", 2, set},
	make_opcode! {0xED, "SET 5,L", "5", "L", 2, set},
	make_opcode! {0xEE, "SET 5,(HL)", "5", "(HL)",  4, set},
	make_opcode! {0xEF, "SET 5,A", "5", "A", 2, set},
	make_opcode! {0xF0, "SET 6,B", "6", "B", 2, set},
	make_opcode! {0xF1, "SET 6,C", "6", "C", 2, set},
	make_opcode! {0xF2, "SET 6,D", "6", "D", 2, set},
	make_opcode! {0xF3, "SET 6,E", "6", "E", 2, set},
	make_opcode! {0xF4, "SET 6,H", "6", "H", 2, set},
	make_opcode! {0xF5, "SET 6,L", "6", "L", 2, set},
	make_opcode! {0xF6, "SET 6,(HL)", "6", "(HL)",  4, set},
	make_opcode! {0xF7, "SET 6,A", "6", "A", 2, set},
	make_opcode! {0xF8, "SET 7,B", "7", "B", 2, set},
	make_opcode! {0xF9, "SET 7,C", "7", "C", 2, set},
	make_opcode! {0xFA, "SET 7,D", "7", "D", 2, set},
	make_opcode! {0xFB, "SET 7,E", "7", "E", 2, set},
	make_opcode! {0xFC, "SET 7,H", "7", "H", 2, set},
	make_opcode! {0xFD, "SET 7,L", "7", "L", 2, set},
	make_opcode! {0xFE, "SET 7,(HL)", "7", "(HL)",  4, set},
	make_opcode! {0xFF, "SET 7,A", "7", "A", 2, set},
]);

fn rlc(c: &mut Cpu, r1: String, _: String) {
	let r = c.load(&r1) as Byte;
	let value = r.rotate_left(1);

	c.reg.F.z = value == 0;
	c.reg.F.n = false;
	c.reg.F.h = false;
	c.reg.F.c = (r & 0x80) == 0x80;

	c.store(&r1, value as Word);
}

fn rrc(c: &mut Cpu, r1: String, _: String) {
	let r = c.load(&r1) as Byte;
	let value = r.rotate_right(1);

	c.reg.F.z = value == 0;
	c.reg.F.n = false;
	c.reg.F.h = false;
	c.reg.F.c = r & 0x01 == 0x01;

	c.store(&r1, value as Word);
}

fn rl(c: &mut Cpu, r1: String, _: String) {
	let r = c.load(&r1) as Byte;
	let carry = c.reg.F.c;
	let mut value = r << 1;

	if carry {
		value += 1;
	}

	c.reg.F.z = value == 0;
	c.reg.F.n = false;
	c.reg.F.h = false;
	c.reg.F.c = r & 0x80 == 0x80;

	c.store(&r1, value as Word);
}

fn rr(c: &mut Cpu, r1: String, _: String) {
	let r = c.load(&r1);
	let carry = c.reg.F.c;
	let mut value = r >> 1;

	if carry {
		value |= 0x80;
	}

	c.reg.F.z = value == 0;
	c.reg.F.n = false;
	c.reg.F.h = false;
	c.reg.F.c = r & 0x01 == 0x01;

	c.store(&r1, value);
}

fn sla(c: &mut Cpu, r1: String, _: String) {
	let r = c.load(&r1) as Byte;
	let value = r << 1;

	c.reg.F.z = value == 0;
	c.reg.F.n = false;
	c.reg.F.h = false;
	c.reg.F.c = r & 0x80 == 0x80;

	c.store(&r1, value as Word);
}

fn sra(c: &mut Cpu, r1: String, _: String) {
	let r = c.load(&r1);
	let bit7 = crate::util::bit(&(r as Byte), &7);
	let mut value = r >> 1;

	if bit7 == 1 {
		value |= 0x80;
	}

	c.reg.F.z = value == 0;
	c.reg.F.n = false;
	c.reg.F.h = false;
	c.reg.F.c = r & 0x01 == 0x01;

	c.store(&r1, value);
}

fn swap(c: &mut Cpu, r1: String, _: String) {
	let r = c.load(&r1);

	let upper = ((r >> 4) & 0x0F) as Byte;
	let lower = (r & 0x0F) as Byte;

	let value = (lower << 4) | upper;

	c.reg.F.z = value == 0;
	c.reg.F.n = false;
	c.reg.F.h = false;
	c.reg.F.c = false;

	c.store(&r1, value as Word);
}

fn srl(c: &mut Cpu, r1: String, _: String) {
	let r = c.load(&r1);
	let value = r >> 1;

	c.reg.F.z = value == 0;
	c.reg.F.n = false;
	c.reg.F.h = false;
	c.reg.F.c = r & 0x01 == 0x01;

	c.store(&r1, value);
}

fn bit(c: &mut Cpu, str_i: String, r1: String) {
	let r = c.load(&r1);
	let i: u8 = str_i.parse().unwrap();
	let bit = crate::util::bit(&(r as Byte), &i);

	c.reg.F.z = bit == 0;
	c.reg.F.n = false;
	c.reg.F.h = true;
}

fn res(c: &mut Cpu, str_i: String, r1: String) {
	let r = c.load(&r1);
	let i: u8 = str_i.parse().unwrap();

	let value = r & !(1 << i);

	c.store(&r1, value);
}

fn set(c: &mut Cpu, str_i: String, r1: String) {
	let r = c.load(&r1);
	let i: u8 = str_i.parse().unwrap();

	let value = r | (1 << i);

	c.store(&r1, value);
}
