extern crate rstest;
#[cfg(test)]
extern crate speculate;

mod common;
use rstest::*;
use rust_boy::cpu::*;
use rust_boy::opcode::*;
use rust_boy::types::*;
use rust_boy::util::*;
use speculate::speculate;

speculate! {
    describe "LD" {
        describe "ld" {
            struct Args {
                opcode: Byte,
                r1: String,
                r2: String,
            }
            #[rstest(arg,
                case(Args{opcode: 0x01, r1: "BC".to_string(), r2: "dd".to_string()}),
                case(Args{opcode: 0x02, r1: "(BC)".to_string(), r2: "A".to_string()}),
                case(Args{opcode: 0x06, r1: "B".to_string(), r2: "d".to_string()}),
                case(Args{opcode: 0x0A, r1: "A".to_string(), r2: "(BC)".to_string()}),
                case(Args{opcode: 0x0E, r1: "C".to_string(), r2: "d".to_string()}),
                case(Args{opcode: 0x11, r1: "DE".to_string(), r2: "dd".to_string()}),
                case(Args{opcode: 0x12, r1: "(DE)".to_string(), r2: "A".to_string()}),
                case(Args{opcode: 0x16, r1: "D".to_string(), r2: "d".to_string()}),
                case(Args{opcode: 0x1A, r1: "A".to_string(), r2: "(DE)".to_string()}),
                case(Args{opcode: 0x1E, r1: "E".to_string(), r2: "d".to_string()}),
                case(Args{opcode: 0x21, r1: "HL".to_string(), r2: "dd".to_string()}),
                case(Args{opcode: 0x22, r1: "(HLI)".to_string(), r2: "A".to_string()}),
                case(Args{opcode: 0x26, r1: "H".to_string(), r2: "d".to_string()}),
                case(Args{opcode: 0x2A, r1: "A".to_string(), r2: "(HLI)".to_string()}),
                case(Args{opcode: 0x2E, r1: "L".to_string(), r2: "d".to_string()}),
                case(Args{opcode: 0x31, r1: "SP".to_string(), r2: "dd".to_string()}),
                case(Args{opcode: 0x32, r1: "(HLD)".to_string(), r2: "A".to_string()}),
                case(Args{opcode: 0x3A, r1: "A".to_string(), r2: "(HLD)".to_string()}),
                case(Args{opcode: 0x36, r1: "(HL)".to_string(), r2: "d".to_string()}),
                case(Args{opcode: 0x3E, r1: "A".to_string(), r2: "d".to_string()}),
                case(Args{opcode: 0x40, r1: "B".to_string(), r2: "B".to_string()}),
                case(Args{opcode: 0x41, r1: "B".to_string(), r2: "C".to_string()}),
                case(Args{opcode: 0x42, r1: "B".to_string(), r2: "D".to_string()}),
                case(Args{opcode: 0x43, r1: "B".to_string(), r2: "E".to_string()}),
                case(Args{opcode: 0x44, r1: "B".to_string(), r2: "H".to_string()}),
                case(Args{opcode: 0x45, r1: "B".to_string(), r2: "L".to_string()}),
                case(Args{opcode: 0x46, r1: "B".to_string(), r2: "(HL)".to_string()}),
                case(Args{opcode: 0x47, r1: "B".to_string(), r2: "A".to_string()}),
                case(Args{opcode: 0x48, r1: "C".to_string(), r2: "B".to_string()}),
                case(Args{opcode: 0x49, r1: "C".to_string(), r2: "C".to_string()}),
                case(Args{opcode: 0x4A, r1: "C".to_string(), r2: "D".to_string()}),
                case(Args{opcode: 0x4B, r1: "C".to_string(), r2: "E".to_string()}),
                case(Args{opcode: 0x4C, r1: "C".to_string(), r2: "H".to_string()}),
                case(Args{opcode: 0x4D, r1: "C".to_string(), r2: "L".to_string()}),
                case(Args{opcode: 0x4E, r1: "C".to_string(), r2: "(HL)".to_string()}),
                case(Args{opcode: 0x4F, r1: "C".to_string(), r2: "A".to_string()}),
                case(Args{opcode: 0x50, r1: "D".to_string(), r2: "B".to_string()}),
                case(Args{opcode: 0x51, r1: "D".to_string(), r2: "C".to_string()}),
                case(Args{opcode: 0x52, r1: "D".to_string(), r2: "D".to_string()}),
                case(Args{opcode: 0x53, r1: "D".to_string(), r2: "E".to_string()}),
                case(Args{opcode: 0x54, r1: "D".to_string(), r2: "H".to_string()}),
                case(Args{opcode: 0x55, r1: "D".to_string(), r2: "L".to_string()}),
                case(Args{opcode: 0x56, r1: "D".to_string(), r2: "(HL)".to_string()}),
                case(Args{opcode: 0x57, r1: "D".to_string(), r2: "A".to_string()}),
                case(Args{opcode: 0x58, r1: "E".to_string(), r2: "B".to_string()}),
                case(Args{opcode: 0x59, r1: "E".to_string(), r2: "C".to_string()}),
                case(Args{opcode: 0x5A, r1: "E".to_string(), r2: "D".to_string()}),
                case(Args{opcode: 0x5B, r1: "E".to_string(), r2: "E".to_string()}),
                case(Args{opcode: 0x5C, r1: "E".to_string(), r2: "H".to_string()}),
                case(Args{opcode: 0x5D, r1: "E".to_string(), r2: "L".to_string()}),
                case(Args{opcode: 0x5E, r1: "E".to_string(), r2: "(HL)".to_string()}),
                case(Args{opcode: 0x5F, r1: "E".to_string(), r2: "A".to_string()}),
                case(Args{opcode: 0x60, r1: "H".to_string(), r2: "B".to_string()}),
                case(Args{opcode: 0x61, r1: "H".to_string(), r2: "C".to_string()}),
                case(Args{opcode: 0x62, r1: "H".to_string(), r2: "D".to_string()}),
                case(Args{opcode: 0x63, r1: "H".to_string(), r2: "E".to_string()}),
                case(Args{opcode: 0x64, r1: "H".to_string(), r2: "H".to_string()}),
                case(Args{opcode: 0x65, r1: "H".to_string(), r2: "L".to_string()}),
                case(Args{opcode: 0x66, r1: "H".to_string(), r2: "(HL)".to_string()}),
                case(Args{opcode: 0x67, r1: "H".to_string(), r2: "A".to_string()}),
                case(Args{opcode: 0x68, r1: "L".to_string(), r2: "B".to_string()}),
                case(Args{opcode: 0x69, r1: "L".to_string(), r2: "C".to_string()}),
                case(Args{opcode: 0x6A, r1: "L".to_string(), r2: "D".to_string()}),
                case(Args{opcode: 0x6B, r1: "L".to_string(), r2: "E".to_string()}),
                case(Args{opcode: 0x6C, r1: "L".to_string(), r2: "H".to_string()}),
                case(Args{opcode: 0x6D, r1: "L".to_string(), r2: "L".to_string()}),
                case(Args{opcode: 0x6E, r1: "L".to_string(), r2: "(HL)".to_string()}),
                case(Args{opcode: 0x6F, r1: "L".to_string(), r2: "A".to_string()}),
                case(Args{opcode: 0x70, r1: "(HL)".to_string(), r2: "B".to_string()}),
                case(Args{opcode: 0x71, r1: "(HL)".to_string(), r2: "C".to_string()}),
                case(Args{opcode: 0x72, r1: "(HL)".to_string(), r2: "D".to_string()}),
                case(Args{opcode: 0x73, r1: "(HL)".to_string(), r2: "E".to_string()}),
                case(Args{opcode: 0x74, r1: "(HL)".to_string(), r2: "H".to_string()}),
                case(Args{opcode: 0x75, r1: "(HL)".to_string(), r2: "L".to_string()}),
                case(Args{opcode: 0x77, r1: "(HL)".to_string(), r2: "A".to_string()}),
                case(Args{opcode: 0xE0, r1: "(a)".to_string(), r2: "A".to_string()}),
                case(Args{opcode: 0xE2, r1: "(C)".to_string(), r2: "A".to_string()}),
                case(Args{opcode: 0xF0, r1: "A".to_string(), r2: "(a)".to_string()}),
                case(Args{opcode: 0xF2, r1: "A".to_string(), r2: "(C)".to_string()}),
            )]
            fn test(arg: Args) {
                let mut cpu = common::fixture::setup_cpu();

                let opcode = &OPCODES[arg.opcode as usize];
                let want = 0x12;

                if &arg.r2 == "d" {
                    cpu.bus.write(cpu.reg.PC, want);
                } else if &arg.r2 == "dd" {
                    cpu.bus.write(cpu.reg.PC, want);
                    cpu.bus.write(cpu.reg.PC + 1, want + 1);
                } else {
                    cpu.store(&arg.r2, want as Word);
                }

                if arg.r2.contains("HLI") {
                    let value = cpu.reg.r16(&"HL".to_string()) - 1;
                    cpu.reg.r16_mut(&"HL".to_string(), value);
                } else if arg.r2.contains("HLD") {
                    let value = cpu.reg.r16(&"HL".to_string()) + 1;
                    cpu.reg.r16_mut(&"HL".to_string(), value);
                }

                let handler = &opcode.handler;
                handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                if arg.r1.contains("HLI") {
                    let value = cpu.reg.r16(&"HL".to_string()) - 1;
                    cpu.reg.r16_mut(&"HL".to_string(), value);
                } else if arg.r1.contains("HLD") {
                    let value = cpu.reg.r16(&"HL".to_string()) + 1;
                    cpu.reg.r16_mut(&"HL".to_string(), value);
                }

                assert_eq!(opcode.r1, arg.r1);
                assert_eq!(opcode.r2, arg.r2);
                
                let rr_array = ["BC", "DE", "HL", "AF", "HL", "SP"];
                if rr_array.contains(&opcode.r1.as_str()) {
                    assert_eq!(cpu.load(&opcode.r1), bytes_2_word(want, want + 1));
                } else {
                    assert_eq!(cpu.load(&opcode.r1), want as Word);
                }
            }
        }
    }
    describe "JP" {
        describe "jpa16" {

        }
    }
}
