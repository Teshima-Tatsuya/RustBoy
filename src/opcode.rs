use crate::{cpu::Cpu, traits::*, types::*, util::*};
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
		write!(
			f,
			"OpCode: {:02X} {} {} {} {}",
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
	make_opcode! {0x01, "LD BC,d16", "BC", "",   3, ldr16d16},
	make_opcode! {0x02, "LD (BC),A", "BC", "A",  2, ldm16r},
	make_opcode! {0x03, "INC BC", "BC", "",   2, incr16},
	make_opcode! {0x04, "INC B", "B", "",   1, incr},
	make_opcode! {0x05, "DEC B", "B", "",   1, decr},
	make_opcode! {0x06, "LD B,d8", "B", "",   2, ldrd},
	make_opcode! {0x07, "RLCA", "",  "",   1, empty},
	make_opcode! {0x08, "LD (a16),SP", "",  "SP",  5, lda16r16},
	make_opcode! {0x09, "ADD HL,BC", "HL", "BC",  2, addr16r16},
	make_opcode! {0x0A, "LD A,(BC)", "A", "(BC)",  2, ld},
	make_opcode! {0x0B, "DEC BC", "BC", "",   2, decr16},
	make_opcode! {0x0C, "INC C", "C", "",   1, incr},
	make_opcode! {0x0D, "DEC C", "C", "",   1, decr},
	make_opcode! {0x0E, "LD C,d8", "C", "",   2, ldrd},
	make_opcode! {0x0F, "RRCA", "",  "",   1, empty},
	make_opcode! {0x10, "STOP 0", "",  "",   1, stop},
	make_opcode! {0x11, "LD DE,d16", "DE", "",   3, ldr16d16},
	make_opcode! {0x12, "LD (DE),A", "DE", "A",  2, ldm16r},
	make_opcode! {0x13, "INC DE", "DE", "",   2, incr16},
	make_opcode! {0x14, "INC D", "D", "",   1, incr},
	make_opcode! {0x15, "DEC D", "D", "",   1, decr},
	make_opcode! {0x16, "LD D,d8", "D", "",   2, ldrd},
	make_opcode! {0x17, "RLA", "",  "",   1, empty},
	make_opcode! {0x18, "JR r8", "",  "",   3, jrr8},
	make_opcode! {0x19, "ADD HL,DE", "HL", "DE",  2, addr16r16},
	make_opcode! {0x1A, "LD A,(DE)", "A", "(DE)",  2, ld},
	make_opcode! {0x1B, "DEC DE", "DE", "",   2, decr16},
	make_opcode! {0x1C, "INC E", "E", "",   1, incr},
	make_opcode! {0x1D, "DEC E", "E", "",   1, decr},
	make_opcode! {0x1E, "LD E,d8", "E", "",   2, ldrd},
	make_opcode! {0x1F, "RRA", "",  "",   1, empty},
	make_opcode! {0x20, "JR NZ,r8", "Z", "",   2, jrnfr8},
	make_opcode! {0x21, "LD HL,d16", "HL", "",   3, ldr16d16},
	make_opcode! {0x22, "LD (HL+),A", "HLI", "A",  2, ldm16r},
	make_opcode! {0x23, "INC HL", "HL", "",   2, incr16},
	make_opcode! {0x24, "INC H", "H", "",   1, incr},
	make_opcode! {0x25, "DEC H", "H", "",   1, decr},
	make_opcode! {0x26, "LD H,d8", "H", "",   2, ldrd},
	make_opcode! {0x27, "DAA", "",  "",   1, empty},
	make_opcode! {0x28, "JR Z,r8", "Z", "",   2, jrfr8},
	make_opcode! {0x29, "ADD HL,HL", "HL", "HL",  2, addr16r16},
	make_opcode! {0x2A, "LD A,(HL+)", "A", "(HLI)",  2, ld},
	make_opcode! {0x2B, "DEC HL", "HL", "",   2, decr16},
	make_opcode! {0x2C, "INC L", "L", "",   1, incr},
	make_opcode! {0x2D, "DEC L", "L", "",   1, decr},
	make_opcode! {0x2E, "LD L,d8", "L", "",   2, ldrd},
	make_opcode! {0x2F, "CPL", "",  "",   1, cpl},
	make_opcode! {0x30, "JR NC,r8", "C", "",   2, jrnfr8},
	make_opcode! {0x31, "LD SP,d16", "SP", "",   3, ldr16d16},
	make_opcode! {0x32, "LD (HL-),A", "HLD", "A",  2, ldm16r},
	make_opcode! {0x33, "INC SP", "SP", "",   2, incr16},
	make_opcode! {0x34, "INC (HL)", "HL", "",   3, incm16},
	make_opcode! {0x35, "DEC (HL)", "HL", "",   3, decm16},
	make_opcode! {0x36, "LD (HL),d8", "HL", "",   3, ldm16d},
	make_opcode! {0x37, "SCF", "",  "",   1, scf},
	make_opcode! {0x38, "JR C,r8", "C", "",   2, jrfr8},
	make_opcode! {0x39, "ADD HL,SP", "HL", "SP",  2, addr16r16},
	make_opcode! {0x3A, "LD A,(HL-)", "A", "(HLD)",  2, ld},
	make_opcode! {0x3B, "DEC SP", "SP", "",   2, decr16},
	make_opcode! {0x3C, "INC A", "A", "",   1, incr},
	make_opcode! {0x3D, "DEC A", "A", "",   1, decr},
	make_opcode! {0x3E, "LD A,d8", "A", "",   2, ldrd},
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
	make_opcode! {0x70, "LD (HL), B", "HL", "B",  2, ldm16r},
	make_opcode! {0x71, "LD (HL), C", "HL", "C",  2, ldm16r},
	make_opcode! {0x72, "LD (HL), D", "HL", "D",  2, ldm16r},
	make_opcode! {0x73, "LD (HL), E", "HL", "E",  2, ldm16r},
	make_opcode! {0x74, "LD (HL), H", "HL", "H",  2, ldm16r},
	make_opcode! {0x75, "LD (HL), L", "HL", "L",  2, ldm16r},
	make_opcode! {0x76, "HALT", "",  "",   1, empty},
	make_opcode! {0x77, "LD (HL), A", "HL", "A",  2, ldm16r},
	make_opcode! {0x78, "LD A, B", "A", "B",  1, ld},
	make_opcode! {0x79, "LD A, C", "A", "C",  1, ld},
	make_opcode! {0x7A, "LD A, D", "A", "D",  1, ld},
	make_opcode! {0x7B, "LD A, E", "A", "E",  1, ld},
	make_opcode! {0x7C, "LD A, H", "A", "H",  1, ld},
	make_opcode! {0x7D, "LD A, L", "A", "L",  1, ld},
	make_opcode! {0x7E, "LD A, (HL)", "A", "(HL)",  2, ld},
	make_opcode! {0x7F, "LD A, A", "A", "A",  1, ld},
	make_opcode! {0x80, "ADD A, B", "A", "B",  1, addr},
	make_opcode! {0x81, "ADD A, C", "A", "C",  1, addr},
	make_opcode! {0x82, "ADD A, D", "A", "D",  1, addr},
	make_opcode! {0x83, "ADD A, E", "A", "E",  1, addr},
	make_opcode! {0x84, "ADD A, H", "A", "H",  1, addr},
	make_opcode! {0x85, "ADD A, L", "A", "L",  1, addr},
	make_opcode! {0x86, "ADD A, (HL)", "A", "(HL)",  2, add_hl},
	make_opcode! {0x87, "ADD A, A", "A", "A",  1, addr},
	make_opcode! {0x88, "ADC A, B", "A", "B",  1, adcr},
	make_opcode! {0x89, "ADC A, C", "A", "C",  1, adcr},
	make_opcode! {0x8A, "ADC A, D", "A", "D",  1, adcr},
	make_opcode! {0x8B, "ADC A, E", "A", "E",  1, adcr},
	make_opcode! {0x8C, "ADC A, H", "A", "H",  1, adcr},
	make_opcode! {0x8D, "ADC A, L", "A", "L",  1, adcr},
	make_opcode! {0x8E, "ADC A, (HL)", "A", "HL",  2, adcm16},
	make_opcode! {0x8F, "ADC A, A", "A", "A",  1, adcr},
	make_opcode! {0x90, "SUB B", "B", "",   1, subr},
	make_opcode! {0x91, "SUB C", "C", "",   1, subr},
	make_opcode! {0x92, "SUB D", "D", "",   1, subr},
	make_opcode! {0x93, "SUB E", "E", "",   1, subr},
	make_opcode! {0x94, "SUB H", "H", "",   1, subr},
	make_opcode! {0x95, "SUB L", "L", "",   1, subr},
	make_opcode! {0x96, "SUB (HL)", "HL", "",   2, sub_hl},
	make_opcode! {0x97, "SUB A", "A", "",   1, subr},
	make_opcode! {0x98, "SBC A, B", "A", "B",  1, sbcr},
	make_opcode! {0x99, "SBC A, C", "A", "C",  1, sbcr},
	make_opcode! {0x9A, "SBC A, D", "A", "D",  1, sbcr},
	make_opcode! {0x9B, "SBC A, E", "A", "E",  1, sbcr},
	make_opcode! {0x9C, "SBC A, H", "A", "H",  1, sbcr},
	make_opcode! {0x9D, "SBC A, L", "A", "L",  1, sbcr},
	make_opcode! {0x9E, "SBC A, (HL)", "A", "HL",  2, sbcm16},
	make_opcode! {0x9F, "SBC A, A", "A", "A",  1, sbcr},
	make_opcode! {0xA0, "AND B", "B", "",   1, andr},
	make_opcode! {0xA1, "AND C", "C", "",   1, andr},
	make_opcode! {0xA2, "AND D", "D", "",   1, andr},
	make_opcode! {0xA3, "AND E", "E", "",   1, andr},
	make_opcode! {0xA4, "AND H", "H", "",   1, andr},
	make_opcode! {0xA5, "AND L", "L", "",   1, andr},
	make_opcode! {0xA6, "AND (HL)", "HL", "",   2, and_hl},
	make_opcode! {0xA7, "AND A", "A", "",   1, andr},
	make_opcode! {0xA8, "XOR B", "B", "",   1, xorr},
	make_opcode! {0xA9, "XOR C", "C", "",   1, xorr},
	make_opcode! {0xAA, "XOR D", "D", "",   1, xorr},
	make_opcode! {0xAB, "XOR E", "E", "",   1, xorr},
	make_opcode! {0xAC, "XOR H", "H", "",   1, xorr},
	make_opcode! {0xAD, "XOR L", "L", "",   1, xorr},
	make_opcode! {0xAE, "XOR (HL)", "HL", "",   2, xor_hl},
	make_opcode! {0xAF, "XOR A", "A", "",   1, xorr},
	make_opcode! {0xB0, "OR B", "B", "",   1, orr},
	make_opcode! {0xB1, "OR C", "C", "",   1, orr},
	make_opcode! {0xB2, "OR D", "D", "",   1, orr},
	make_opcode! {0xB3, "OR E", "E", "",   1, orr},
	make_opcode! {0xB4, "OR H", "H", "",   1, orr},
	make_opcode! {0xB5, "OR L", "L", "",   1, orr},
	make_opcode! {0xB6, "OR (HL)", "HL", "",   2, or_hl},
	make_opcode! {0xB7, "OR A", "A", "",   1, orr},
	make_opcode! {0xB8, "CP B", "B", "",   1, cpr},
	make_opcode! {0xB9, "CP C", "C", "",   1, cpr},
	make_opcode! {0xBA, "CP D", "D", "",   1, cpr},
	make_opcode! {0xBB, "CP E", "E", "",   1, cpr},
	make_opcode! {0xBC, "CP H", "H", "",   1, cpr},
	make_opcode! {0xBD, "CP L", "L", "",   1, cpr},
	make_opcode! {0xBE, "CP (HL)", "HL", "",   2, cp_hl},
	make_opcode! {0xBF, "CP A", "A", "",   1, cpr},
	make_opcode! {0xC0, "RET NZ", "Z", "",   2, retnf},
	make_opcode! {0xC1, "POP BC", "BC", "",   3, pop},
	make_opcode! {0xC2, "JP NZ,a16", "Z", "",   3, jpnfa16},
	make_opcode! {0xC3, "JP a16", "",  "",   4, jpa16},
	make_opcode! {0xC4, "CALL NZ,a16", "Z", "",   3, callnf},
	make_opcode! {0xC5, "PUSH BC", "BC", "",   4, push},
	make_opcode! {0xC6, "ADD A,d8", "A", "",   2, addd8},
	make_opcode! {0xC7, "RST 00H", "0x00", "",   4, rst},
	make_opcode! {0xC8, "RET Z", "Z", "",   2, retf},
	make_opcode! {0xC9, "RET", "",  "",   4, ret},
	make_opcode! {0xCA, "JP Z,a16", "Z", "",   3, jpfa16},
	make_opcode! {0xCB, "PREFIX CB", "",  "",   1, empty},
	make_opcode! {0xCC, "CALL Z,a16", "Z", "",   3, callf},
	make_opcode! {0xCD, "CALL a16", "",  "",   6, call},
	make_opcode! {0xCE, "ADC A,d8", "A", "",   2, adcd},
	make_opcode! {0xCF, "RST 08H", "0x08", "",   4, rst},
	make_opcode! {0xD0, "RET NC", "C", "",   2, retnf},
	make_opcode! {0xD1, "POP DE", "DE", "",   3, pop},
	make_opcode! {0xD2, "JP NC,a16", "C", "",   3, jpnfa16},
	make_opcode! {0xD3, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xD4, "CALL NC,a16", "C", "",   3, callnf},
	make_opcode! {0xD5, "PUSH DE", "DE", "",   4, push},
	make_opcode! {0xD6, "SUB d8", "",  "",   2, subd8},
	make_opcode! {0xD7, "RST 10H", "0x10", "",   4, rst},
	make_opcode! {0xD8, "RET C", "C", "",   2, retf},
	make_opcode! {0xD9, "RETI", "",  "",   4, reti},
	make_opcode! {0xDA, "JP C,a16", "C", "",   3, jpfa16},
	make_opcode! {0xDB, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xDC, "CALL C,a16", "C", "",   3, callf},
	make_opcode! {0xDD, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xDE, "SBC A,d8", "A", "",   2, sbcd},
	make_opcode! {0xDF, "RST 18H", "0x18", "",   4, rst},
	make_opcode! {0xE0, "LDH (a8),A", "",  "A",  3, ldar},
	make_opcode! {0xE1, "POP HL", "HL", "",   3, pop},
	make_opcode! {0xE2, "LD (C),A", "C", "A",  2, ldmr},
	make_opcode! {0xE3, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xE4, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xE5, "PUSH HL", "HL", "",   4, push},
	make_opcode! {0xE6, "AND d8", "",  "",   2, andd8},
	make_opcode! {0xE7, ":w
	 20H", "0x20", "",   4, rst},
	make_opcode! {0xE8, "ADD SP,r8", "SP", "",   4, addr16d},
	make_opcode! {0xE9, "JP (HL)", "HL", "",   1, jpm16},
	make_opcode! {0xEA, "LD (a16),A", "",  "A",  4, lda16r},
	make_opcode! {0xEB, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xEC, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xED, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xEE, "XOR d8", "",  "",   2, xord8},
	make_opcode! {0xEF, "RST 28H", "0x28", "",   4, rst},
	make_opcode! {0xF0, "LDH A,(a8)", "A", "",   3, ldra},
	make_opcode! {0xF1, "POP AF", "AF",  "",   3, pop},
	make_opcode! {0xF2, "LD A,(C)", "A", "(C)",  2, ld},
	make_opcode! {0xF3, "DI", "",  "",   1, di},
	make_opcode! {0xF4, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xF5, "PUSH AF", "AF",  "",   4, push},
	make_opcode! {0xF6, "OR d8", "",  "",   2, ord8},
	make_opcode! {0xF7, "RST 30H", "0x30", "",   4, rst},
	make_opcode! {0xF8, "LD HL,SP+r8", "HL", "SP",  3, ldr16r16d},
	make_opcode! {0xF9, "LD SP,HL", "SP", "HL",  2, ldr16r16},
	make_opcode! {0xFA, "LD A,(a16)", "A", "",   4, ldra16},
	make_opcode! {0xFB, "EI", "",  "",   1, empty},
	make_opcode! {0xFC, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xFD, "EMPTY", "",  "",   0,  empty},
	make_opcode! {0xFE, "CP d8", "",  "",   2, cpd8},
	make_opcode! {0xFF, "RST 38H", "0x38", "",   4, rst},
]);

fn empty(_: &mut Cpu, _: String, _: String) {
	unreachable!("this is empty!");
}

fn nop(_: &mut Cpu, _: String, _: String) {
	println!("nop");
}

fn stop(_: &mut Cpu, _: String, _: String) {
	todo!("stop impl");
}

fn ld(c: &mut Cpu, r1: String, r2: String) {
	let value = c.load(&r2);
	c.store(&r1, value);
}

// LD r1, d8
fn ldrd(c: &mut Cpu, r1: String, _: String) {
	let value = c.fetch();
	c.reg.r_mut(&r1, value);
}

// LDH R,(a8)
fn ldra(c: &mut Cpu, r1: String, _: String) {
	let addr = c.fetch();
	let value = c.bus.read(bytes_2_word(0xFF as Byte, addr));
	c.reg.r_mut(&r1, value);
}

fn ldra16(c: &mut Cpu, r1: String, _: String) {
	let addr = c.fetch16();
	let value = c.bus.read(addr);
	c.reg.r_mut(&r1, value);
}

// fn ldr16(r16, r16d, d16)
// LD r1, r2
fn ldr16r16(c: &mut Cpu, r1: String, r2: String) {
	let value = c.reg.r16(&r2);
	c.reg.r16_mut(&r1, value);
}

// LD r1, r2+d
fn ldr16r16d(c: &mut Cpu, r1: String, r2: String) {
	// d = c.fetch();
	// sp = c.reg.R16(SP);
	// v = sp + types.Addr((int8(d)));
	// c.reg.setR16(int(R1), types.Addr(v));

	// carry = sp ^ types.Addr(d) ^ (sp + types.Addr(d))

	// c.reg.setZNHC(false, false, carry&0x10 == 0x10, carry&0x100 == 0x100)
}

// LD r1, d16
fn ldr16d16(c: &mut Cpu, r1: String, _: String) {
	let value = c.fetch16();
	c.reg.r16_mut(&r1, value)
}

// fn ldm(r)

// LD (C), A
fn ldmr(c: &mut Cpu, r1: String, r2: String) {
	let addr = bytes_2_word(0xFF, c.reg.r(&r1));
	c.bus.write(addr, c.reg.r(&r2));
}

// fn ldm16(r, d)

// LD (r1), r2
fn ldm16r(c: &mut Cpu, r1: String, r2: String) {
	let value = c.reg.r(&r2);
	let addr = c.reg.r16(&r1);
	c.bus.write(addr, value);
}

// LD (HL),d8
fn ldm16d(c: &mut Cpu, r1: String, _: String) {
	let value = c.fetch();
	let addr = c.reg.r16(&r1);
	c.bus.write(addr, value)
}

// fn lda(r)

fn ldar(c: &mut Cpu, _: String, r2: String) {
	let addr = bytes_2_word(0xFF, c.fetch());
	c.bus.write(addr, c.reg.r(&r2));
}

// fn lda16(r, r16)

fn lda16r(c: &mut Cpu, _: String, r2: String) {
	let addr = c.fetch16();
	c.bus.write(addr, c.reg.r(&r2));
}

fn lda16r16(c: &mut Cpu, _: String, r2: String) {
	let addr = c.fetch16();
	let r16 = c.reg.r16(&r2);
	c.bus.write(addr, extract_lower(r16));
	c.bus.write(addr + 1, extract_upper(r16));
}

// arithmetic
fn incr(c: &mut Cpu, r8: String, _: String) {
	let r = c.reg.r(&&r8);

	let v = r.wrapping_add(0x01);

	c.reg.F.z = v == 0;
	c.reg.F.n = false;
	c.reg.F.h = (r ^ v) & 0x10 != 0;

	c.reg.r_mut(&&r8, v);
}

fn incr16(c: &mut Cpu, r16: String, _: String) {
	let r = c.reg.r16(&r16);
	let v = r.wrapping_add(1);
	c.reg.r16_mut(&r16, v);
}

fn incm16(c: &mut Cpu, r16: String, _: String) {
	let r = c.bus.read(c.reg.r16(&r16));
	let v = r.wrapping_add(1);

	c.reg.F.z = v == 0;
	c.reg.F.n = false;
	c.reg.F.h = (r ^ v) & 0x10 != 0;

	c.bus.write(c.reg.r16(&r16), v);
}

fn decr(c: &mut Cpu, r8: String, _: String) {
	let r = c.reg.r(&r8);
	let v = r.wrapping_sub(0x01);

	c.reg.F.z = v == 0;
	c.reg.F.n = true;
	c.reg.F.h = (r ^ v) & 0x10 != 0;

	c.reg.r_mut(&r8, v);
}

fn decr16(c: &mut Cpu, r16: String, _: String) {
	let r = c.reg.r16(&r16);
	let v = r.wrapping_sub(1);
	c.reg.r16_mut(&r16, v);
}

fn decm16(c: &mut Cpu, r16: String, _: String) {
	let r = c.bus.read(c.reg.r16(&r16));
	let v = r.wrapping_sub(1);

	c.reg.F.z = v == 0;
	c.reg.F.n = true;
	c.reg.F.h = (r ^ v) & 0x10 != 0;

	c.bus.write(c.reg.r16(&r16), v);
}

fn _and(c: &mut Cpu, buf: Byte) {
	let a = c.reg.r(&"A".to_string());
	let value = a & buf;
	c.reg.r_mut(&"A".to_string(), value);

	let z = if a == 0 { 1 } else { 0 };
	let znhc = (z << 7) & 0xA0;
	c.reg.F.unpack(znhc);
}

fn andr(c: &mut Cpu, r: String, _: String) {
	let buf = c.reg.r(&r);
	_and(c, buf)
}

fn and_hl(c: &mut Cpu, r: String, _: String) {
	let buf = c.bus.read(c.reg.r16(&r));
	_and(c, buf)
}

fn andd8(c: &mut Cpu, _: String, _: String) {
	let buf = c.fetch();
	_and(c, buf)
}

fn _or(c: &mut Cpu, buf: Byte) {
	let a = c.reg.r(&"A".to_string());
	let value = a | buf;
	c.reg.r_mut(&"A".to_string(), value);

	let z = if a == 0 { 1 } else { 0 };
	let znhc = (z << 7) & 0x80;
	c.reg.F.unpack(znhc);
}

fn orr(c: &mut Cpu, r: String, _: String) {
	let buf = c.reg.r(&r);
	_or(c, buf)
}

fn or_hl(c: &mut Cpu, r: String, _: String) {
	let buf = c.bus.read(c.reg.r16(&r));
	_or(c, buf)
}

fn ord8(c: &mut Cpu, _: String, _: String) {
	let buf = c.fetch();
	_or(c, buf)
}

fn _xor(c: &mut Cpu, buf: Byte) {
	let a = c.reg.r(&"A".to_string());
	let value = a ^ buf;
	c.reg.r_mut(&"A".to_string(), value);

	let z = if a == 0 { 1 } else { 0 };
	let znhc = (z << 7) & 0x80;
	c.reg.F.unpack(znhc);
}

fn xorr(c: &mut Cpu, r: String, _: String) {
	let buf = c.reg.r(&r);
	_xor(c, buf)
}

fn xor_hl(c: &mut Cpu, r: String, _: String) {
	let buf = c.bus.read(c.reg.r16(&r));
	_xor(c, buf)
}

fn xord8(c: &mut Cpu, _: String, _: String) {
	let buf = c.fetch();
	_xor(c, buf)
}

fn _cp(cpu: &mut Cpu, v: Byte) {
	let a = cpu.reg.r(&"A".to_string());

	let z = if a == v { 1 } else { 0 };
	let h = if a & 0x0F < v & 0x0F { 1 } else { 0 };
	let c = if a < v { 1 } else { 0 };
	let znhc = (z << 7) | 0x40 | (h << 6) | (c << 4);
	cpu.reg.F.unpack(znhc);
}

fn cpr(c: &mut Cpu, r: String, _: String) {
	let v = c.reg.r(&r);
	_cp(c, v)
}

fn cp_hl(c: &mut Cpu, r: String, _: String) {
	let v = c.bus.read(c.reg.r16(&r));
	_cp(c, v)
}

fn cpd8(c: &mut Cpu, _: String, _: String) {
	let v = c.fetch();
	_cp(c, v)
}

fn _add(c: &mut Cpu, b: Byte) {
	let (v, overflow) = c.reg.A.overflowing_add(b);

	c.reg.F.z = v == 0;
	c.reg.F.n = false;
	c.reg.F.h = (c.reg.A ^ b ^ v) & 0x10 != 0;
	c.reg.F.c = overflow;

	c.reg.A = v;
}

fn addr(c: &mut Cpu, _: String, r: String) {
	let r = c.reg.r(&r);
	_add(c, r)
}

fn addr16r16(c: &mut Cpu, r1: String, r2: String) {
	let a = c.reg.r16(&r1);
	let b = c.reg.r16(&r2);

	let (v, overflow) = a.overflowing_add(b);

	c.reg.F.n = false;
	c.reg.F.h = (a ^ b ^ v) & 0x1000 != 0;
	c.reg.F.c = overflow;

	c.reg.r16_mut(&r1, v);
}

fn addr16d(c: &mut Cpu, r: String, _: String) {
	// v1 = c.reg.R16(r16)
	// v2 = int8(c.fetch())

	// v = uint32(v1) + uint32(v2)

	// carry = uint32(v1) ^ uint32(v2) ^ v

	// c.reg.setR16(r16, types.Addr(v))
	// c.reg.setZNHC(false, false, carry&0x10 == 0x10, carry&0x100 == 0x100)
}

fn add_hl(c: &mut Cpu, _: String, r: String) {
	let v = c.bus.read(c.reg.r16(&r));
	_add(c, v);
}

fn addd8(c: &mut Cpu, _: String, _: String) {
	let v = c.fetch();
	_add(c, v);
}

fn _adc(c: &mut Cpu, r: Byte) {
	// a = c.reg.R[A]
	// carry = c.reg.isSet(C)

	// v = a + r + byte(util.Bool2Int8(carry))

	// c.reg.R[A] = v
	// flag_h = a&0x0F+r&0x0F+byte(util.Bool2Int8(carry)) > 0x0F
	// flag_c = uint(a&0xFF)+uint(r&0xFF)+uint(util.Bool2Int8(carry)) > 0xFF
	// c.reg.setZNHC(v == 0, false, flag_h, flag_c)
}

// ADC A,R
fn adcr(c: &mut Cpu, _: String, r2: String) {
	let r = c.reg.r(&r2);
	_adc(c, r)
}

fn adcm16(c: &mut Cpu, _: String, r2: String) {
	let r = c.bus.read(c.reg.r16(&r2));
	_adc(c, r)
}

fn adcd(c: &mut Cpu, _: String, _: String) {
	let r = c.fetch();
	_adc(c, r)
}

fn _sub(c: &mut Cpu, b: Byte) {
	// a = c.reg.R[A]
	// v = a - b
	// carryBits = a ^ b ^ v
	// flag_h = carryBits&(1<<4) != 0
	// flag_c = a < v

	// c.reg.R[A] = v
	// c.reg.setZNHC(byte(v) == 0, true, flag_h, flag_c)
}

fn subr(c: &mut Cpu, r: String, _: String) {
	let v = c.reg.r(&r);
	_sub(c, v);
}

fn sub_hl(c: &mut Cpu, r: String, _: String) {
	let v = c.bus.read(c.reg.r16(&r));
	_sub(c, v);
}

fn subd8(c: &mut Cpu, _: String, _: String) {
	let r = c.fetch();
	_sub(c, r)
}

fn _sbc(c: &mut Cpu, r: Byte) {
	// a = c.reg.R[A]
	// carry = util.Bool2Int8(c.reg.isSet(C))

	// v = a - (r + byte(carry))
	// c.reg.R[A] = byte(v)

	// flag_h = a&0x0F < r&0x0F+byte(carry)
	// flag_c = uint16(a) < uint16(r)+uint16(carry)
	// c.reg.setZNHC(byte(v) == 0, true, flag_h, flag_c)
}

// SBC A,R
fn sbcr(c: &mut Cpu, _: String, r2: String) {
	let r = c.reg.r(&r2);
	_sbc(c, r);
}

fn sbcm16(c: &mut Cpu, _: String, r2: String) {
	let r = c.reg.r16(&r2);
	let v = c.bus.read(r);
	_sbc(c, v);
}

fn sbcd(c: &mut Cpu, _: String, _: String) {
	let r = c.fetch();
	_sbc(c, r);
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
	let flag_b;
	match flag_str {
		"Z" => flag_b = c.reg.F.z,
		"N" => flag_b = c.reg.F.n,
		"H" => flag_b = c.reg.F.h,
		"C" => flag_b = c.reg.F.c,
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
	let flag_b;
	match flag_str {
		"Z" => flag_b = c.reg.F.z,
		"N" => flag_b = c.reg.F.n,
		"H" => flag_b = c.reg.F.h,
		"C" => flag_b = c.reg.F.c,
		_ => unreachable!(),
	}
	if !flag_b {
		_jp(c, addr)
	}
}

// JP (r16)
fn jpm16(c: &mut Cpu, r1: String, _: String) {
	let addr = c.reg.r16(&r1);
	_jp(c, addr);
}

// ret
fn ret(c: &mut Cpu, _: String, _: String) {
	c.pop_pc()
}

fn retf(c: &mut Cpu, r: String, _: String) {
	if c.reg.F.f(r) {
		c.pop_pc()
	}
}

fn retnf(c: &mut Cpu, r: String, _: String) {
	if !c.reg.F.f(r) {
		c.pop_pc()
	}
}

fn reti(c: &mut Cpu, _: String, _: String) {
	c.pop_pc();
	// c.IRQ.Enable();
}

// -----jr-----
fn _jr(c: &mut Cpu, addr: i8) {
	c.reg.PC = ((c.reg.PC as i32) + (addr as i32)) as Word;
}

// r8 is a signed data, which are added to PC
fn jrr8(c: &mut Cpu, _: String, _: String) {
	let n = c.fetch();
	_jr(c, n as i8)
}

// r8 is a signed data, which are added to PC
fn jrfr8(c: &mut Cpu, flag: String, _: String) {
	let n = c.fetch();
	// flag is set
	if c.reg.F.f(flag) {
		_jr(c, n as i8)
	}
}

// r8 is a signed data, which are added to PC
fn jrnfr8(c: &mut Cpu, flag: String, _: String) {
	let n = c.fetch();
	// flag is not set
	if !c.reg.F.f(flag) {
		_jr(c, n as i8)
	}
}

// -----rst------

// RST n
// push and jump to n
fn rst(c: &mut Cpu, n: String, _: String) {
	c.push_pc();
	c.reg.PC = n.parse::<i32>().unwrap() as Word
}

// -----push-----
fn push(c: &mut Cpu, r: String, _: String) {
	let buf = c.reg.r16(&r);
	let upper = extract_upper(buf as Word);
	let lower = extract_lower(buf as Word);
	c.push(upper);
	c.push(lower);
}

// -----pop------
fn pop(c: &mut Cpu, r: String, _: String) {
	let mut lower = c.pop();
	let upper = c.pop();

	if r == "AF" {
		lower &= 0xF0
	}

	let value = ((upper as i16) << 8 | (lower as i16)) as Word;
	c.reg.r16_mut(&r, value);
}

// -----call-----
fn _call(c: &mut Cpu, dest: Word) {
	c.push_pc();
	c.reg.PC = dest
}

fn call(c: &mut Cpu, _: String, _: String) {
	let dest = c.fetch16();
	_call(c, dest)
}

fn callf(c: &mut Cpu, flag: String, _: String) {
	let dest = c.fetch16();
	if c.reg.F.f(flag) {
		_call(c, dest)
	}
}

fn callnf(c: &mut Cpu, flag: String, _: String) {
	let dest = c.fetch16();
	if !c.reg.F.f(flag) {
		_call(c, dest)
	}
}

// -----misc-----

fn cpl(c: &mut Cpu, _: String, _: String) {
	let a = c.reg.r(&"A".to_string());
	c.reg.r_mut(&"A".to_string(), a ^ a);

	let znhc = c.reg.F.pack() | 0x60;
	c.reg.F.unpack(znhc);
}

fn scf(c: &mut Cpu, _: String, _: String) {
	let znhc = (c.reg.F.pack() & 0x80) | 0x10;
	c.reg.F.unpack(znhc);
}

fn ccf(cpu: &mut Cpu, _: String, _: String) {
	let c = if !cpu.reg.F.f("C".to_string()) { 1 } else { 0 };
	let znhc = (cpu.reg.F.pack() & 0x80) | (c << 4);
	cpu.reg.F.unpack(znhc);
}

fn di(c: &mut Cpu, _: String, _: String) {
	warn!("not implemented di")
	//	c.IRQ.Disable()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_register_set_get() {}
}
