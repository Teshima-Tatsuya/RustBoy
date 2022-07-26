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
    describe "OPCODE" {
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
                    case(Args{opcode: 0x08, r1: "(aa)".to_string(), r2: "SP".to_string()}),
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
                    case(Args{opcode: 0xEA, r1: "(aa)".to_string(), r2: "A".to_string()}),
                    case(Args{opcode: 0xF0, r1: "A".to_string(), r2: "(a)".to_string()}),
                    case(Args{opcode: 0xF2, r1: "A".to_string(), r2: "(C)".to_string()}),
                    case(Args{opcode: 0xF9, r1: "SP".to_string(), r2: "HL".to_string()}),
                    case(Args{opcode: 0xFA, r1: "A".to_string(), r2: "(aa)".to_string()}),
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
                    if rr_array.contains(&opcode.r1.as_str()) && &opcode.r2.as_str() != &"HL" {
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
    describe "CB_OPCODE" {
        describe "rlc" {
            struct Args {
                opcode: Byte,
                r1: String,
            }
            #[rstest(arg,
                case(Args{opcode: 0x00, r1: "B".to_string()}),
                case(Args{opcode: 0x01, r1: "C".to_string()}),
                case(Args{opcode: 0x02, r1: "D".to_string()}),
                case(Args{opcode: 0x03, r1: "E".to_string()}),
                case(Args{opcode: 0x04, r1: "H".to_string()}),
                case(Args{opcode: 0x05, r1: "L".to_string()}),
                case(Args{opcode: 0x06, r1: "(HL)".to_string()}),
                case(Args{opcode: 0x07, r1: "A".to_string()}),
            )]
            fn test(arg: Args) {
                let mut cpu = common::fixture::setup_cpu();

                let opcode = &CB_OPCODES[arg.opcode as usize];
                assert_eq!(opcode.r1, arg.r1);

                cpu = cb_helper(cpu, opcode, 0b10010000, 0b00100001, 0x10);
                cpu = cb_helper(cpu, opcode, 0b00000000, 0b00000000, 0x80);
                cpu = cb_helper(cpu, opcode, 0b00100000, 0b01000000, 0x00);
            }
        }

        describe "rrc" {
            struct Args {
                opcode: Byte,
                r1: String,
            }
            #[rstest(arg,
                case(Args{opcode: 0x08, r1: "B".to_string()}),
                case(Args{opcode: 0x09, r1: "C".to_string()}),
                case(Args{opcode: 0x0A, r1: "D".to_string()}),
                case(Args{opcode: 0x0B, r1: "E".to_string()}),
                case(Args{opcode: 0x0C, r1: "H".to_string()}),
                case(Args{opcode: 0x0D, r1: "L".to_string()}),
                case(Args{opcode: 0x0E, r1: "(HL)".to_string()}),
                case(Args{opcode: 0x0F, r1: "A".to_string()}),
            )]
            fn test(arg: Args) {
                let mut cpu = common::fixture::setup_cpu();

                let opcode = &CB_OPCODES[arg.opcode as usize];
                assert_eq!(opcode.r1, arg.r1);

                cpu = cb_helper(cpu, opcode, 0b10010001, 0b11001000, 0x10);
                cpu = cb_helper(cpu, opcode, 0b00000000, 0b00000000, 0x80);
                cpu = cb_helper(cpu, opcode, 0b00100000, 0b00010000, 0x00);
            }
        }

        describe "rl" {
            struct Args {
                opcode: Byte,
                r1: String,
            }
            #[rstest(arg,
                case(Args{opcode: 0x10, r1: "B".to_string()}),
                case(Args{opcode: 0x11, r1: "C".to_string()}),
                case(Args{opcode: 0x12, r1: "D".to_string()}),
                case(Args{opcode: 0x13, r1: "E".to_string()}),
                case(Args{opcode: 0x14, r1: "H".to_string()}),
                case(Args{opcode: 0x15, r1: "L".to_string()}),
                case(Args{opcode: 0x16, r1: "(HL)".to_string()}),
                case(Args{opcode: 0x17, r1: "A".to_string()}),
            )]
            fn test(arg: Args) {
                let mut cpu = common::fixture::setup_cpu();

                let opcode = &CB_OPCODES[arg.opcode as usize];
                assert_eq!(opcode.r1, arg.r1);

                cpu.reg.F.c = true;
                cpu = cb_helper(cpu, opcode, 0b10010000, 0b00100001, 0x10);
                cpu.reg.F.c = false;
                cpu = cb_helper(cpu, opcode, 0b10010000, 0b00100000, 0x10);
                cpu.reg.F.c = true;
                cpu = cb_helper(cpu, opcode, 0b00000000, 0b00000001, 0x00);
                cpu.reg.F.c = false;
                cpu = cb_helper(cpu, opcode, 0b00000000, 0b00000000, 0x80);
                cpu.reg.F.c = true;
                cpu = cb_helper(cpu, opcode, 0b00100000, 0b01000001, 0x00);
                cpu.reg.F.c = false;
                cpu = cb_helper(cpu, opcode, 0b00100000, 0b01000000, 0x00);
            }
        }

        describe "rr" {
            struct Args {
                opcode: Byte,
                r1: String,
            }
            #[rstest(arg,
                case(Args{opcode: 0x18, r1: "B".to_string()}),
                case(Args{opcode: 0x19, r1: "C".to_string()}),
                case(Args{opcode: 0x1A, r1: "D".to_string()}),
                case(Args{opcode: 0x1B, r1: "E".to_string()}),
                case(Args{opcode: 0x1C, r1: "H".to_string()}),
                case(Args{opcode: 0x1D, r1: "L".to_string()}),
                case(Args{opcode: 0x1E, r1: "(HL)".to_string()}),
                case(Args{opcode: 0x1F, r1: "A".to_string()}),
            )]
            fn test(arg: Args) {
                let mut cpu = common::fixture::setup_cpu();

                let opcode = &CB_OPCODES[arg.opcode as usize];
                assert_eq!(opcode.r1, arg.r1);

                cpu.reg.F.c = true;
                cpu = cb_helper(cpu, opcode, 0b10010001, 0b11001000, 0x10);
                cpu.reg.F.c = false;
                cpu = cb_helper(cpu, opcode, 0b10010001, 0b01001000, 0x10);
                cpu.reg.F.c = true;
                cpu = cb_helper(cpu, opcode, 0b00000000, 0b10000000, 0x00);
                cpu.reg.F.c = false;
                cpu = cb_helper(cpu, opcode, 0b00000000, 0b00000000, 0x80);
                cpu.reg.F.c = true;
                cpu = cb_helper(cpu, opcode, 0b00100000, 0b10010000, 0x00);
                cpu.reg.F.c = false;
                cpu = cb_helper(cpu, opcode, 0b00100000, 0b00010000, 0x00);
            }
        }

        describe "sla" {
            struct Args {
                opcode: Byte,
                r1: String,
            }
            #[rstest(arg,
                case(Args{opcode: 0x20, r1: "B".to_string()}),
                case(Args{opcode: 0x21, r1: "C".to_string()}),
                case(Args{opcode: 0x22, r1: "D".to_string()}),
                case(Args{opcode: 0x23, r1: "E".to_string()}),
                case(Args{opcode: 0x24, r1: "H".to_string()}),
                case(Args{opcode: 0x25, r1: "L".to_string()}),
                case(Args{opcode: 0x26, r1: "(HL)".to_string()}),
                case(Args{opcode: 0x27, r1: "A".to_string()}),
            )]
            fn test(arg: Args) {
                let mut cpu = common::fixture::setup_cpu();

                let opcode = &CB_OPCODES[arg.opcode as usize];
                assert_eq!(opcode.r1, arg.r1);

                cpu = cb_helper(cpu, opcode, 0b10010000, 0b00100000, 0x10);
                cpu = cb_helper(cpu, opcode, 0b00000000, 0b00000000, 0x80);
                cpu = cb_helper(cpu, opcode, 0b00100000, 0b01000000, 0x00);
            }
        }

        describe "sra" {
            struct Args {
                opcode: Byte,
                r1: String,
            }
            #[rstest(arg,
                case(Args{opcode: 0x28, r1: "B".to_string()}),
                case(Args{opcode: 0x29, r1: "C".to_string()}),
                case(Args{opcode: 0x2A, r1: "D".to_string()}),
                case(Args{opcode: 0x2B, r1: "E".to_string()}),
                case(Args{opcode: 0x2C, r1: "H".to_string()}),
                case(Args{opcode: 0x2D, r1: "L".to_string()}),
                case(Args{opcode: 0x2E, r1: "(HL)".to_string()}),
                case(Args{opcode: 0x2F, r1: "A".to_string()}),
            )]
            fn test(arg: Args) {
                let mut cpu = common::fixture::setup_cpu();

                let opcode = &CB_OPCODES[arg.opcode as usize];
                assert_eq!(opcode.r1, arg.r1);

                cpu = cb_helper(cpu, opcode, 0b10010001, 0b11001000, 0x10);
                cpu = cb_helper(cpu, opcode, 0b00000000, 0b00000000, 0x80);
                cpu = cb_helper(cpu, opcode, 0b00100000, 0b00010000, 0x00);
            }
        }

        describe "swap" {
            struct Args {
                opcode: Byte,
                r1: String,
            }
            #[rstest(arg,
                case(Args{opcode: 0x30, r1: "B".to_string()}),
                case(Args{opcode: 0x31, r1: "C".to_string()}),
                case(Args{opcode: 0x32, r1: "D".to_string()}),
                case(Args{opcode: 0x33, r1: "E".to_string()}),
                case(Args{opcode: 0x34, r1: "H".to_string()}),
                case(Args{opcode: 0x35, r1: "L".to_string()}),
                case(Args{opcode: 0x36, r1: "(HL)".to_string()}),
                case(Args{opcode: 0x37, r1: "A".to_string()}),
            )]
            fn test(arg: Args) {
                let mut cpu = common::fixture::setup_cpu();

                let opcode = &CB_OPCODES[arg.opcode as usize];
                assert_eq!(opcode.r1, arg.r1);

                cpu = cb_helper(cpu, opcode, 0b10010100, 0b01001001, 0x00);
                cpu = cb_helper(cpu, opcode, 0b00000000, 0b00000000, 0x80);
            }
        }

        describe "srl" {
            struct Args {
                opcode: Byte,
                r1: String,
            }
            #[rstest(arg,
                case(Args{opcode: 0x38, r1: "B".to_string()}),
                case(Args{opcode: 0x39, r1: "C".to_string()}),
                case(Args{opcode: 0x3A, r1: "D".to_string()}),
                case(Args{opcode: 0x3B, r1: "E".to_string()}),
                case(Args{opcode: 0x3C, r1: "H".to_string()}),
                case(Args{opcode: 0x3D, r1: "L".to_string()}),
                case(Args{opcode: 0x3E, r1: "(HL)".to_string()}),
                case(Args{opcode: 0x3F, r1: "A".to_string()}),
            )]
            fn test(arg: Args) {
                let mut cpu = common::fixture::setup_cpu();

                let opcode = &CB_OPCODES[arg.opcode as usize];
                assert_eq!(opcode.r1, arg.r1);

                cpu = cb_helper(cpu, opcode, 0b10010001, 0b01001000, 0x10);
                cpu = cb_helper(cpu, opcode, 0b00000000, 0b00000000, 0x80);
                cpu = cb_helper(cpu, opcode, 0b00100000, 0b00010000, 0x00);
            }
        }

        describe "bit" {
            struct Args {
                opcode: Byte,
                i: String,
                r1: String,
            }
            #[rstest(arg,
                case(Args{opcode: 0x40, i: "0".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0x41, i: "0".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0x42, i: "0".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0x43, i: "0".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0x44, i: "0".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0x45, i: "0".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0x46, i: "0".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0x47, i: "0".to_string(), r1: "A".to_string()}),
                case(Args{opcode: 0x48, i: "1".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0x49, i: "1".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0x4A, i: "1".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0x4B, i: "1".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0x4C, i: "1".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0x4D, i: "1".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0x4E, i: "1".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0x4F, i: "1".to_string(), r1: "A".to_string()}),
                case(Args{opcode: 0x50, i: "2".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0x51, i: "2".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0x52, i: "2".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0x53, i: "2".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0x54, i: "2".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0x55, i: "2".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0x56, i: "2".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0x57, i: "2".to_string(), r1: "A".to_string()}),
                case(Args{opcode: 0x58, i: "3".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0x59, i: "3".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0x5A, i: "3".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0x5B, i: "3".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0x5C, i: "3".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0x5D, i: "3".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0x5E, i: "3".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0x5F, i: "3".to_string(), r1: "A".to_string()}),
                case(Args{opcode: 0x60, i: "4".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0x61, i: "4".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0x62, i: "4".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0x63, i: "4".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0x64, i: "4".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0x65, i: "4".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0x66, i: "4".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0x67, i: "4".to_string(), r1: "A".to_string()}),
                case(Args{opcode: 0x68, i: "5".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0x69, i: "5".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0x6A, i: "5".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0x6B, i: "5".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0x6C, i: "5".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0x6D, i: "5".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0x6E, i: "5".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0x6F, i: "5".to_string(), r1: "A".to_string()}),
                case(Args{opcode: 0x70, i: "6".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0x71, i: "6".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0x72, i: "6".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0x73, i: "6".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0x74, i: "6".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0x75, i: "6".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0x76, i: "6".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0x77, i: "6".to_string(), r1: "A".to_string()}),
                case(Args{opcode: 0x78, i: "7".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0x79, i: "7".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0x7A, i: "7".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0x7B, i: "7".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0x7C, i: "7".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0x7D, i: "7".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0x7E, i: "7".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0x7F, i: "7".to_string(), r1: "A".to_string()}),
            )]
            fn test(arg: Args) {
                let mut cpu = common::fixture::setup_cpu();

                let opcode = &CB_OPCODES[arg.opcode as usize];
                assert_eq!(opcode.r1, arg.i);
                assert_eq!(opcode.r2, arg.r1);

                let mut before: Byte;
                let i: Byte = arg.i.parse().unwrap();

                before = 0b00000000u8 | 1 << i;
                cpu.store(&opcode.r2, before as Word);
                let handler = &opcode.handler;
                handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                assert_eq!(cpu.reg.F.pack(), 0b00100000);

                before = 0b00000000u8;
                cpu.store(&opcode.r2, before as Word);
                let handler = &opcode.handler;
                handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                assert_eq!(cpu.reg.F.pack(), 0b10100000);
            }
        }
        
        describe "res" {
            struct Args {
                opcode: Byte,
                i: String,
                r1: String,
            }
            #[rstest(arg,
                case(Args{opcode: 0x80, i: "0".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0x81, i: "0".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0x82, i: "0".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0x83, i: "0".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0x84, i: "0".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0x85, i: "0".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0x86, i: "0".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0x87, i: "0".to_string(), r1: "A".to_string()}),
                case(Args{opcode: 0x88, i: "1".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0x89, i: "1".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0x8A, i: "1".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0x8B, i: "1".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0x8C, i: "1".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0x8D, i: "1".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0x8E, i: "1".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0x8F, i: "1".to_string(), r1: "A".to_string()}),
                case(Args{opcode: 0x90, i: "2".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0x91, i: "2".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0x92, i: "2".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0x93, i: "2".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0x94, i: "2".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0x95, i: "2".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0x96, i: "2".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0x97, i: "2".to_string(), r1: "A".to_string()}),
                case(Args{opcode: 0x98, i: "3".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0x99, i: "3".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0x9A, i: "3".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0x9B, i: "3".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0x9C, i: "3".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0x9D, i: "3".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0x9E, i: "3".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0x9F, i: "3".to_string(), r1: "A".to_string()}),
                case(Args{opcode: 0xA0, i: "4".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0xA1, i: "4".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0xA2, i: "4".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0xA3, i: "4".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0xA4, i: "4".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0xA5, i: "4".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0xA6, i: "4".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0xA7, i: "4".to_string(), r1: "A".to_string()}),
                case(Args{opcode: 0xA8, i: "5".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0xA9, i: "5".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0xAA, i: "5".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0xAB, i: "5".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0xAC, i: "5".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0xAD, i: "5".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0xAE, i: "5".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0xAF, i: "5".to_string(), r1: "A".to_string()}),
                case(Args{opcode: 0xB0, i: "6".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0xB1, i: "6".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0xB2, i: "6".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0xB3, i: "6".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0xB4, i: "6".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0xB5, i: "6".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0xB6, i: "6".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0xB7, i: "6".to_string(), r1: "A".to_string()}),
                case(Args{opcode: 0xB8, i: "7".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0xB9, i: "7".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0xBA, i: "7".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0xBB, i: "7".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0xBC, i: "7".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0xBD, i: "7".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0xBE, i: "7".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0xBF, i: "7".to_string(), r1: "A".to_string()}),
            )]
            fn test(arg: Args) {
                let mut cpu = common::fixture::setup_cpu();

                let opcode = &CB_OPCODES[arg.opcode as usize];
                assert_eq!(opcode.r1, arg.i);
                assert_eq!(opcode.r2, arg.r1);

                let mut before: Byte;
                let i: Byte = arg.i.parse().unwrap();

                before = 0b00000000u8 | 1 << i;
                cpu.store(&opcode.r2, before as Word);
                let handler = &opcode.handler;
                handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                assert_eq!(cpu.load(&opcode.r2), 0b00000000);

                before = 0b00000000u8;
                cpu.store(&opcode.r2, before as Word);
                let handler = &opcode.handler;
                handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                assert_eq!(cpu.load(&opcode.r2), 0b00000000);
            }
        }
    }
}

fn cb_helper(mut cpu: Cpu, opcode: &OpCode, before: Byte, want: Byte, flag: Byte) -> Cpu {
    cpu.store(&opcode.r1, before as Word);
    let handler = &opcode.handler;
    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());

    assert_eq!(cpu.reg.F.pack(),flag);
    let rr_array = ["BC", "DE", "HL", "AF", "HL", "SP"];
    if rr_array.contains(&opcode.r1.as_str()) {
        assert_eq!(cpu.load(&opcode.r1), bytes_2_word(want, want + 1));
    } else {
        assert_eq!(cpu.load(&opcode.r1), want as Word);
    }

    cpu
}