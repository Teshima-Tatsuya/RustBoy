use crate::cpu::Cpu;
use crate::types::*;
use log::warn;
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
		let mut result = String::from("OpCode:");
		write!(
			f,
			"OpCode: {} {} {} {} {}",
			self.code, self.mnemonic, self.r1, self.r2, self.cycles
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
	make_opcode! {0x01, "LD BC,d16", "BC", "",   3, empty},
	make_opcode! {0x02, "LD (BC),A", "BC", "A",  2, empty},
	make_opcode! {0x03, "INC BC", "BC", "",   2, empty},
	make_opcode! {0x04, "INC B", "B", "",   1, empty},
	make_opcode! {0x05, "DEC B", "B", "",   1, empty},
	make_opcode! {0x06, "LD B,d8", "B", "",   2, empty},
	make_opcode! {0x07, "RLCA", "",  "",   1, empty},
	make_opcode! {0x08, "LD (a16),SP", "",  "SP",  5, empty},
	make_opcode! {0x09, "ADD HL,BC", "HL", "BC",  2, empty},
	make_opcode! {0x0A, "LD A,(BC)", "A", "BC",  2, empty},
	make_opcode! {0x0B, "DEC BC", "BC", "",   2, empty},
	make_opcode! {0x0C, "INC C", "C", "",   1, empty},
	make_opcode! {0x0D, "DEC C", "C", "",   1, empty},
	make_opcode! {0x0E, "LD C,d8", "C", "",   2, empty},
	make_opcode! {0x0F, "empty", "",  "",   1, empty},
	make_opcode! {0x10, "empty 0", "",  "",   1, empty},
	make_opcode! {0x11, "LD DE,d16", "DE", "",   3, empty},
	make_opcode! {0x12, "LD (DE),A", "DE", "A",  2, empty},
	make_opcode! {0x13, "INC DE", "DE", "",   2, empty},
	make_opcode! {0x14, "INC D", "D", "",   1, empty},
	make_opcode! {0x15, "DEC D", "D", "",   1, empty},
	make_opcode! {0x16, "LD D,d8", "D", "",   2, empty},
	make_opcode! {0x17, "RLA", "",  "",   1, empty},
	make_opcode! {0x18, "JR r8", "",  "",   3, empty},
	make_opcode! {0x19, "ADD HL,DE", "HL", "DE",  2, empty},
	make_opcode! {0x1A, "LD A,(DE)", "A", "DE",  2, empty},
	make_opcode! {0x1B, "DEC DE", "DE", "",   2, empty},
	make_opcode! {0x1C, "INC E", "E", "",   1, empty},
	make_opcode! {0x1D, "DEC E", "E", "",   1, empty},
	make_opcode! {0x1E, "LD E,d8", "E", "",   2, empty},
	make_opcode! {0x1F, "RRA", "",  "",   1, empty},
	make_opcode! {0x20, "JR NZ,r8", "flagZ", "",   2, empty},
	make_opcode! {0x21, "LD HL,d16", "HL", "",   3, empty},
	make_opcode! {0x22, "LD (HL+),A", "HLI", "A",  2, empty},
	make_opcode! {0x23, "INC HL", "HL", "",   2, empty},
	make_opcode! {0x24, "INC H", "H", "",   1, empty},
	make_opcode! {0x25, "DEC H", "H", "",   1, empty},
	make_opcode! {0x26, "LD H,d8", "H", "",   2, empty},
	make_opcode! {0x27, "DAA", "",  "",   1, empty},
	make_opcode! {0x28, "JR Z,r8", "flagZ", "",   2, empty},
	make_opcode! {0x29, "ADD HL,HL", "HL", "HL",  2, empty},
	make_opcode! {0x2A, "LD A,(HL+)", "A", "HLI",  2, empty},
	make_opcode! {0x2B, "DEC HL", "HL", "",   2, empty},
	make_opcode! {0x2C, "INC L", "L", "",   1, empty},
	make_opcode! {0x2D, "DEC L", "L", "",   1, empty},
	make_opcode! {0x2E, "LD L,d8", "L", "",   2, empty},
	make_opcode! {0x2F, "CPL", "",  "",   1, empty},
	make_opcode! {0x30, "JR NC,r8", "flagC", "",   2, empty},
	make_opcode! {0x31, "LD SP,d16", "SP", "",   3, empty},
	make_opcode! {0x32, "LD (HL-),A", "HLD", "A",  2, empty},
	make_opcode! {0x33, "INC SP", "SP", "",   2, empty},
	make_opcode! {0x34, "INC (HL)", "HL", "",   3, empty},
	make_opcode! {0x35, "DEC (HL)", "HL", "",   3, empty},
	make_opcode! {0x36, "LD (HL),d8", "HL", "",   3, empty},
	make_opcode! {0x37, "SCF", "",  "",   1, empty},
	make_opcode! {0x38, "JR C,r8", "flagC", "",   2, empty},
	make_opcode! {0x39, "ADD HL,SP", "HL", "SP",  2, empty},
	make_opcode! {0x3A, "LD A,(HL-)", "A", "HLD",  2, empty},
	make_opcode! {0x3B, "DEC SP", "SP", "",   2, empty},
	make_opcode! {0x3C, "INC A", "A", "",   1, empty},
	make_opcode! {0x3D, "DEC A", "A", "",   1, empty},
	make_opcode! {0x3E, "LD A,d8", "A", "",   2, empty},
	make_opcode! {0x3F, "CCF", "",  "",   1, empty},
	make_opcode! {0x40, "LD B, B", "B", "B",  1, empty},
	make_opcode! {0x41, "LD B, C", "B", "C",  1, empty},
	make_opcode! {0x42, "LD B, D", "B", "D",  1, empty},
	make_opcode! {0x43, "LD B, E", "B", "E",  1, empty},
	make_opcode! {0x44, "LD B, H", "B", "H",  1, empty},
	make_opcode! {0x45, "LD B, L", "B", "L",  1, empty},
	make_opcode! {0x46, "LD B, HL", "B", "HL",  2, empty},
	make_opcode! {0x47, "LD B, A", "B", "A",  1, empty},
	make_opcode! {0x48, "LD C, B", "C", "B",  1, empty},
	make_opcode! {0x49, "LD C, C", "C", "C",  1, empty},
	make_opcode! {0x4A, "LD C, D", "C", "D",  1, empty},
	make_opcode! {0x4B, "LD C, E", "C", "E",  1, empty},
	make_opcode! {0x4C, "LD C, H", "C", "H",  1, empty},
	make_opcode! {0x4D, "LD C, L", "C", "L",  1, empty},
	make_opcode! {0x4E, "LD C, HL", "C", "HL",  2, empty},
	make_opcode! {0x4F, "LD C, A", "C", "A",  1, empty},
	make_opcode! {0x50, "LD D, B", "D", "B",  1, empty},
	make_opcode! {0x51, "LD D, C", "D", "C",  1, empty},
	make_opcode! {0x52, "LD D, D", "D", "D",  1, empty},
	make_opcode! {0x53, "LD D, E", "D", "E",  1, empty},
	make_opcode! {0x54, "LD D, H", "D", "H",  1, empty},
	make_opcode! {0x55, "LD D, L", "D", "L",  1, empty},
	make_opcode! {0x56, "LD D, HL", "D", "HL",  2, empty},
	make_opcode! {0x57, "LD D, A", "D", "A",  1, empty},
	make_opcode! {0x58, "LD E, B", "E", "B",  1, empty},
	make_opcode! {0x59, "LD E, C", "E", "C",  1, empty},
	make_opcode! {0x5A, "LD E, D", "E", "D",  1, empty},
	make_opcode! {0x5B, "LD E, E", "E", "E",  1, empty},
	make_opcode! {0x5C, "LD E, H", "E", "H",  1, empty},
	make_opcode! {0x5D, "LD E, L", "E", "L",  1, empty},
	make_opcode! {0x5E, "LD E, HL", "E", "HL",  2, empty},
	make_opcode! {0x5F, "LD E, A", "E", "A",  1, empty},
	make_opcode! {0x60, "LD H, B", "H", "B",  1, empty},
	make_opcode! {0x61, "LD H, C", "H", "C",  1, empty},
	make_opcode! {0x62, "LD H, D", "H", "D",  1, empty},
	make_opcode! {0x63, "LD H, E", "H", "E",  1, empty},
	make_opcode! {0x64, "LD H, H", "H", "H",  1, empty},
	make_opcode! {0x65, "LD H, L", "H", "L",  1, empty},
	make_opcode! {0x66, "LD H, HL", "H", "HL",  2, empty},
	make_opcode! {0x67, "LD H, A", "H", "A",  1, empty},
	make_opcode! {0x68, "LD L, B", "L", "B",  1, empty},
	make_opcode! {0x69, "LD L, C", "L", "C",  1, empty},
	make_opcode! {0x6A, "LD L, D", "L", "D",  1, empty},
	make_opcode! {0x6B, "LD L, E", "L", "E",  1, empty},
	make_opcode! {0x6C, "LD L, H", "L", "H",  1, empty},
	make_opcode! {0x6D, "LD L, L", "L", "L",  1, empty},
	make_opcode! {0x6E, "LD L, HL", "L", "HL",  2, empty},
	make_opcode! {0x6F, "LD L, A", "L", "A",  1, empty},
	make_opcode! {0x70, "LD (HL), B", "HL", "B",  2, empty},
	make_opcode! {0x71, "LD (HL), C", "HL", "C",  2, empty},
	make_opcode! {0x72, "LD (HL), D", "HL", "D",  2, empty},
	make_opcode! {0x73, "LD (HL), E", "HL", "E",  2, empty},
	make_opcode! {0x74, "LD (HL), H", "HL", "H",  2, empty},
	make_opcode! {0x75, "LD (HL), L", "HL", "L",  2, empty},
	make_opcode! {0x76, "HALT", "",  "",   1, empty},
	make_opcode! {0x77, "LD (HL), A", "HL", "A",  2, empty},
	make_opcode! {0x78, "LD A, B", "A", "B",  1, empty},
	make_opcode! {0x79, "LD A, C", "A", "C",  1, empty},
	make_opcode! {0x7A, "LD A, D", "A", "D",  1, empty},
	make_opcode! {0x7B, "LD A, E", "A", "E",  1, empty},
	make_opcode! {0x7C, "LD A, H", "A", "H",  1, empty},
	make_opcode! {0x7D, "LD A, L", "A", "L",  1, empty},
	make_opcode! {0x7E, "LD A, (HL)", "A", "HL",  2, empty},
	make_opcode! {0x7F, "LD A, A", "A", "A",  1, empty},
	make_opcode! {0x80, "ADD A, B", "A", "B",  1, empty},
	make_opcode! {0x81, "ADD A, C", "A", "C",  1, empty},
	make_opcode! {0x82, "ADD A, D", "A", "D",  1, empty},
	make_opcode! {0x83, "ADD A, E", "A", "E",  1, empty},
	make_opcode! {0x84, "ADD A, H", "A", "H",  1, empty},
	make_opcode! {0x85, "ADD A, L", "A", "L",  1, empty},
	make_opcode! {0x86, "ADD A, (HL)", "A", "HL",  2, empty},
	make_opcode! {0x87, "ADD A, A", "A", "A",  1, empty},
	make_opcode! {0x88, "ADC A, B", "A", "B",  1, empty},
	make_opcode! {0x89, "ADC A, C", "A", "C",  1, empty},
	make_opcode! {0x8A, "ADC A, D", "A", "D",  1, empty},
	make_opcode! {0x8B, "ADC A, E", "A", "E",  1, empty},
	make_opcode! {0x8C, "ADC A, H", "A", "H",  1, empty},
	make_opcode! {0x8D, "ADC A, L", "A", "L",  1, empty},
	make_opcode! {0x8E, "ADC A, (HL)", "A", "HL",  2, empty},
	make_opcode! {0x8F, "ADC A, A", "A", "A",  1, empty},
	make_opcode! {0x90, "SUB B", "B", "",   1, empty},
	make_opcode! {0x91, "SUB C", "C", "",   1, empty},
	make_opcode! {0x92, "SUB D", "D", "",   1, empty},
	make_opcode! {0x93, "SUB E", "E", "",   1, empty},
	make_opcode! {0x94, "SUB H", "H", "",   1, empty},
	make_opcode! {0x95, "SUB L", "L", "",   1, empty},
	make_opcode! {0x96, "SUB (HL)", "HL", "",   2, empty},
	make_opcode! {0x97, "SUB A", "A", "",   1, empty},
	make_opcode! {0x98, "SBC A, B", "A", "B",  1, empty},
	make_opcode! {0x99, "SBC A, C", "A", "C",  1, empty},
	make_opcode! {0x9A, "SBC A, D", "A", "D",  1, empty},
	make_opcode! {0x9B, "SBC A, E", "A", "E",  1, empty},
	make_opcode! {0x9C, "SBC A, H", "A", "H",  1, empty},
	make_opcode! {0x9D, "SBC A, L", "A", "L",  1, empty},
	make_opcode! {0x9E, "SBC A, (HL)", "A", "HL",  2, empty},
	make_opcode! {0x9F, "SBC A, A", "A", "A",  1, empty},
	make_opcode! {0xA0, "AND B", "B", "",   1, empty},
	make_opcode! {0xA1, "AND C", "C", "",   1, empty},
	make_opcode! {0xA2, "AND D", "D", "",   1, empty},
	make_opcode! {0xA3, "AND E", "E", "",   1, empty},
	make_opcode! {0xA4, "AND H", "H", "",   1, empty},
	make_opcode! {0xA5, "AND L", "L", "",   1, empty},
	make_opcode! {0xA6, "AND (HL)", "HL", "",   2, empty},
	make_opcode! {0xA7, "AND A", "A", "",   1, empty},
	make_opcode! {0xA8, "XOR B", "B", "",   1, empty},
	make_opcode! {0xA9, "XOR C", "C", "",   1, empty},
	make_opcode! {0xAA, "XOR D", "D", "",   1, empty},
	make_opcode! {0xAB, "XOR E", "E", "",   1, empty},
	make_opcode! {0xAC, "XOR H", "H", "",   1, empty},
	make_opcode! {0xAD, "XOR L", "L", "",   1, empty},
	make_opcode! {0xAE, "XOR (HL)", "HL", "",   2, empty},
	make_opcode! {0xAF, "XOR A", "A", "",   1, empty},
	make_opcode! {0xB0, "OR B", "B", "",   1, empty},
	make_opcode! {0xB1, "OR C", "C", "",   1, empty},
	make_opcode! {0xB2, "OR D", "D", "",   1, empty},
	make_opcode! {0xB3, "OR E", "E", "",   1, empty},
	make_opcode! {0xB4, "OR H", "H", "",   1, empty},
	make_opcode! {0xB5, "OR L", "L", "",   1, empty},
	make_opcode! {0xB6, "OR (HL)", "HL", "",   2, empty},
	make_opcode! {0xB7, "OR A", "A", "",   1, empty},
	make_opcode! {0xB8, "CP B", "B", "",   1, empty},
	make_opcode! {0xB9, "CP C", "C", "",   1, empty},
	make_opcode! {0xBA, "CP D", "D", "",   1, empty},
	make_opcode! {0xBB, "CP E", "E", "",   1, empty},
	make_opcode! {0xBC, "CP H", "H", "",   1, empty},
	make_opcode! {0xBD, "CP L", "L", "",   1, empty},
	make_opcode! {0xBE, "CP (HL)", "HL", "",   2, empty},
	make_opcode! {0xBF, "CP A", "A", "",   1, empty},
	make_opcode! {0xC0, "RET NZ", "flagZ", "",   2, empty},
	make_opcode! {0xC1, "POP BC", "BC", "",   3, empty},
	make_opcode! {0xC2, "JP NZ,a16", "flagZ", "",   3, jpnfa16},
	make_opcode! {0xC3, "JP a16", "",  "",   4, jpa16},
	make_opcode! {0xC4, "CALL NZ,a16", "flagZ", "",   3, empty},
	make_opcode! {0xC5, "PUSH BC", "BC", "",   4, empty},
	make_opcode! {0xC6, "ADD A,d8", "A", "",   2, empty},
	make_opcode! {0xC7, "RST 00H", "0x00", "",   4, empty},
	make_opcode! {0xC8, "RET Z", "flagZ", "",   2, empty},
	make_opcode! {0xC9, "RET", "",  "",   4, empty},
	make_opcode! {0xCA, "JP Z,a16", "flagZ", "",   3, jpfa16},
	make_opcode! {0xCB, "PREFIX CB", "",  "",   1, empty},
	make_opcode! {0xCC, "CALL Z,a16", "flagZ", "",   3, empty},
	make_opcode! {0xCD, "CALL a16", "",  "",   6, empty},
	make_opcode! {0xCE, "ADC A,d8", "A", "",   2, empty},
	make_opcode! {0xCF, "RST 08H", "0x08", "",   4, empty},
	make_opcode! {0xD0, "RET NC", "flagC", "",   2, empty},
	make_opcode! {0xD1, "POP DE", "DE", "",   3, empty},
	make_opcode! {0xD2, "JP NC,a16", "flagC", "",   3, jpnfa16},
	make_opcode! {0xD3, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xD4, "CALL NC,a16", "flagC", "",   3, empty},
	make_opcode! {0xD5, "PUSH DE", "DE", "",   4, empty},
	make_opcode! {0xD6, "SUB d8", "",  "",   2, empty},
	make_opcode! {0xD7, "RST 10H", "0x10", "",   4, empty},
	make_opcode! {0xD8, "RET C", "flagC", "",   2, empty},
	make_opcode! {0xD9, "RETI", "",  "",   4, empty},
	make_opcode! {0xDA, "JP C,a16", "flagC", "",   3, jpfa16},
	make_opcode! {0xDB, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xDC, "CALL C,a16", "flagC", "",   3, empty},
	make_opcode! {0xDD, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xDE, "SBC A,d8", "A", "",   2, empty},
	make_opcode! {0xDF, "RST 18H", "0x18", "",   4, empty},
	make_opcode! {0xE0, "LDH (a8),A", "",  "A",  3, empty},
	make_opcode! {0xE1, "POP HL", "HL", "",   3, empty},
	make_opcode! {0xE2, "LD (C),A", "C", "A",  2, empty},
	make_opcode! {0xE3, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xE4, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xE5, "PUSH HL", "HL", "",   4, empty},
	make_opcode! {0xE6, "AND d8", "",  "",   2, empty},
	make_opcode! {0xE7, "RST 20H", "0x20", "",   4, empty},
	make_opcode! {0xE8, "ADD SP,r8", "SP", "",   4, empty},
	make_opcode! {0xE9, "JP (HL)", "HL", "",   1, jpm16},
	make_opcode! {0xEA, "LD (a16),A", "",  "A",  4, empty},
	make_opcode! {0xEB, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xEC, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xED, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xEE, "XOR d8", "",  "",   2, empty},
	make_opcode! {0xEF, "RST 28H", "0x28", "",   4, empty},
	make_opcode! {0xF0, "LDH A,(a8)", "A", "",   3, empty},
	make_opcode! {0xF1, "POP AF", "AF",  "",   3, empty},
	make_opcode! {0xF2, "LD A,(C)", "A", "C",  2, empty},
	make_opcode! {0xF3, "DI", "",  "",   1, di},
	make_opcode! {0xF4, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xF5, "PUSH AF", "AF",  "",   4, empty},
	make_opcode! {0xF6, "OR d8", "",  "",   2, empty},
	make_opcode! {0xF7, "RST 30H", "0x30", "",   4, empty},
	make_opcode! {0xF8, "LD HL,SP+r8", "HL", "SP",  3, empty},
	make_opcode! {0xF9, "LD SP,HL", "SP", "HL",  2, empty},
	make_opcode! {0xFA, "LD A,(a16)", "A", "",   4, empty},
	make_opcode! {0xFB, "EI", "",  "",   1, empty},
	make_opcode! {0xFC, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xFD, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xFE, "CP d8", "",  "",   2, empty},
	make_opcode! {0xFF, "RST 38H", "0x38", "",   4, empty},
]);

fn empty(c: &mut Cpu, _: String, _: String) {
	unreachable!("this is empty!");
}

fn nop(_: &mut Cpu, _: String, _: String) {
	println!("nop");
}

// jp
fn _jp(c: &mut Cpu, addr: Word) {
	c.reg.PC = addr
}

// JP a16
fn jpa16(c: &mut Cpu, _: String, _: String) {
	let addr = c.fetch16();
	_jp(c, addr)
}

// JP flag, a16
// jump when flag = 1
fn jpfa16(c: &mut Cpu, flag: String, _: String) {
	let addr = c.fetch16();

	let flag_str: &str = &flag;
	let mut flag_b = false;
	match flag_str {
		"flagZ" => flag_b = c.reg.F.z,
		"flagN" => flag_b = c.reg.F.n,
		"flagH" => flag_b = c.reg.F.h,
		"flagC" => flag_b = c.reg.F.c,
		_ => unreachable!(),
	}

	if flag_b {
		_jp(c, addr)
	}
}

// JP Nflag, a16
// jump when flag = 0
fn jpnfa16(c: &mut Cpu, flag: String, _: String) {
	let addr = c.fetch16();

	let flag_str: &str = &flag;
	let mut flag_b = false;
	match flag_str {
		"flagZ" => flag_b = c.reg.F.z,
		"flagN" => flag_b = c.reg.F.n,
		"flagH" => flag_b = c.reg.F.h,
		"flagC" => flag_b = c.reg.F.c,
		_ => unreachable!(),
	}
	if !flag_b {
		_jp(c, addr)
	}
}

// JP (r16)
fn jpm16(c: &mut Cpu, R1: String, _: String) {
	warn!("not implemented jpm16")
	//	_jp(c, c.reg.R16(int(R1)))
}

fn di(c: &mut Cpu, _: String, _: String) {
	warn!("not implemented di")
	//	c.IRQ.Disable()
}
