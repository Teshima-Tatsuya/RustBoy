extern crate rstest;
#[cfg(test)]
extern crate speculate;

mod common;
use rstest::*;
use rust_boy::constant::*;
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
                        assert_eq!(cpu.load(&opcode.r1), bytes_2_word(want + 1, want));
                    } else {
                        assert_eq!(cpu.load(&opcode.r1), want as Word);
                    }
                }
            }
        }

        describe "JP" {
            describe "jp" {
                struct Args {
                    opcode: Byte,
                    r1: String,
                    r2: String,
                }
                #[rstest(arg,
                    case(Args{opcode: 0xC3, r1: "aa".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xE9, r1: "HL".to_string(), r2: "".to_string()}),
                )]
                fn test(arg: Args) {
                    let mut cpu = common::fixture::setup_cpu();

                    let opcode = &OPCODES[arg.opcode as usize];
                    assert_eq!(opcode.r1, arg.r1);
                    assert_eq!(opcode.r2, arg.r2);

                    let want: Word = 0x1234;
                    if arg.r1.as_str() == "HL" {
                        cpu.store(&"HL".to_string(), want);
                    } else {
                        cpu.bus.write(cpu.reg.PC,  0x34);
                        cpu.bus.write(cpu.reg.PC + 1,  0x12);
                    }

                    let handler = &opcode.handler;
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());

                    assert_eq!(cpu.load(&"PC".to_string()), want);
                }
            }

            describe "jpf" {
                struct Args {
                    opcode: Byte,
                    r1: String,
                    r2: String,
                }
                #[rstest(arg,
                    case(Args{opcode: 0xC2, r1: "NZ".to_string(), r2: "aa".to_string()}),
                    case(Args{opcode: 0xCA, r1: "Z".to_string(), r2: "aa".to_string()}),
                    case(Args{opcode: 0xD2, r1: "NC".to_string(), r2: "aa".to_string()}),
                    case(Args{opcode: 0xDA, r1: "C".to_string(), r2: "aa".to_string()}),
                )]
                fn test(arg: Args) {
                    let mut cpu = common::fixture::setup_cpu();

                    let opcode = &OPCODES[arg.opcode as usize];
                    assert_eq!(opcode.r1, arg.r1);
                    assert_eq!(opcode.r2, arg.r2);

                    let want: Word = 0x1234;
                    cpu.bus.write(cpu.reg.PC,  0x34);
                    cpu.bus.write(cpu.reg.PC + 1,  0x12);

                    let handler = &opcode.handler;

                    // all true
                    cpu.reg.F.unpack(0xF0);
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    if opcode.r1.as_str() == "NZ" || opcode.r1.as_str() == "NC" {
                        assert_eq!(cpu.load(&"PC".to_string()), cpu.load(&"PC".to_string()));
                    } else {
                        assert_eq!(cpu.load(&"PC".to_string()), want);
                    }

                    // all false
                    cpu.reg.F.unpack(0x00);
                    cpu.reg.PC = cpu.reg.PC - 2;
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    if opcode.r1.as_str() == "NZ" || opcode.r1.as_str() == "NC" {
                        assert_eq!(cpu.load(&"PC".to_string()), want);
                    } else {
                        assert_eq!(cpu.load(&"PC".to_string()), cpu.load(&"PC".to_string()));
                    }
                }
            }
        }

        describe "JR" {
            describe "jr" {
                struct Args {
                    opcode: Byte,
                    r1: String,
                    r2: String,
                }
                #[rstest(arg,
                    case(Args{opcode: 0x18, r1: "d".to_string(), r2: "".to_string()}),
                )]
                fn test(arg: Args) {
                    let mut cpu = common::fixture::setup_cpu();

                    let opcode = &OPCODES[arg.opcode as usize];
                    assert_eq!(opcode.r1, arg.r1);
                    assert_eq!(opcode.r2, arg.r2);

                    let handler = &opcode.handler;

                    // positive
                    cpu.reg.PC = 0x100;
                    cpu.bus.write(cpu.reg.PC,  0x10);
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    assert_eq!(cpu.load(&"PC".to_string()), 0x0111);

                    // nagative
                    cpu.reg.PC = 0x100;
                    cpu.bus.write(cpu.reg.PC,  0xFE); // -2
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    assert_eq!(cpu.load(&"PC".to_string()), 0x00FF);
                }
            }

            describe "jrf" {
                struct Args {
                    opcode: Byte,
                    r1: String,
                    r2: String,
                }
                #[rstest(arg,
                    case(Args{opcode: 0x20, r1: "NZ".to_string(), r2: "d".to_string()}),
                    case(Args{opcode: 0x28, r1: "Z".to_string(), r2: "d".to_string()}),
                    case(Args{opcode: 0x30, r1: "NC".to_string(), r2: "d".to_string()}),
                    case(Args{opcode: 0x38, r1: "C".to_string(), r2: "d".to_string()}),
                )]
                fn test(arg: Args) {
                    let mut cpu = common::fixture::setup_cpu();

                    let opcode = &OPCODES[arg.opcode as usize];
                    assert_eq!(opcode.r1, arg.r1);
                    assert_eq!(opcode.r2, arg.r2);

                    let handler = &opcode.handler;

                    // positive
                    cpu.reg.PC = 0x100;
                    cpu.bus.write(cpu.reg.PC,  0x10);
                    cpu.reg.F.unpack(0xF0);
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    if opcode.r1.as_str() == "NZ" || opcode.r1.as_str() == "NC" {
                        assert_eq!(cpu.load(&"PC".to_string()), 0x0101);
                    } else {
                        assert_eq!(cpu.load(&"PC".to_string()), 0x0111);
                    }

                    // nagative
                    cpu.reg.PC = 0x100;
                    cpu.bus.write(cpu.reg.PC,  0xFE); // -2
                    cpu.reg.F.unpack(0x00);
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    if opcode.r1.as_str() == "NZ" || opcode.r1.as_str() == "NC" {
                        assert_eq!(cpu.load(&"PC".to_string()), 0x00FF);
                    } else {
                        assert_eq!(cpu.load(&"PC".to_string()), 0x0101);
                    }
                }
            }
        }

        describe "INC" {
            describe "incr" {
                struct Args {
                    opcode: Byte,
                    r1: String,
                    r2: String,
                }
                #[rstest(arg,
                    case(Args{opcode: 0x04, r1: "B".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0x0C, r1: "C".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0x14, r1: "D".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0x1C, r1: "E".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0x24, r1: "H".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0x2C, r1: "L".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0x34, r1: "(HL)".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0x3C, r1: "A".to_string(), r2: "".to_string()}),
                )]
                fn test(arg: Args) {
                    let mut cpu = common::fixture::setup_cpu();

                    let opcode = &OPCODES[arg.opcode as usize];
                    assert_eq!(opcode.r1, arg.r1);
                    assert_eq!(opcode.r2, arg.r2);

                    let handler = &opcode.handler;

                    // no carry
                    cpu.reg.F.unpack(0b11110000);
                    cpu.store(&opcode.r1, 0x10);
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    assert_eq!(cpu.load(&opcode.r1), 0x11);
                    assert_eq!(cpu.reg.F.pack(), 0b00010000);

                    // half carry
                    cpu.reg.F.unpack(0b11110000);
                    cpu.store(&opcode.r1, 0x1F);
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    assert_eq!(cpu.load(&opcode.r1), 0x20);
                    assert_eq!(cpu.reg.F.pack(), 0b00110000);

                    // zero
                    cpu.reg.F.unpack(0b11110000);
                    cpu.store(&opcode.r1, 0xFF);
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    assert_eq!(cpu.load(&opcode.r1), 0x00);
                    assert_eq!(cpu.reg.F.pack(), 0b10110000);
                }
            }

            describe "incrr" {
                struct Args {
                    opcode: Byte,
                    r1: String,
                    r2: String,
                }
                #[rstest(arg,
                    case(Args{opcode: 0x03, r1: "BC".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0x13, r1: "DE".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0x23, r1: "HL".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0x33, r1: "SP".to_string(), r2: "".to_string()}),
                )]
                fn test(arg: Args) {
                    let mut cpu = common::fixture::setup_cpu();

                    let opcode = &OPCODES[arg.opcode as usize];
                    assert_eq!(opcode.r1, arg.r1);
                    assert_eq!(opcode.r2, arg.r2);

                    let handler = &opcode.handler;

                    cpu.store(&opcode.r1, 0x1234);
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    assert_eq!(cpu.load(&opcode.r1), 0x1235);
                }
            }
        }

        describe "DEC" {
            describe "decr" {
                struct Args {
                    opcode: Byte,
                    r1: String,
                    r2: String,
                }
                #[rstest(arg,
                    case(Args{opcode: 0x05, r1: "B".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0x0D, r1: "C".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0x15, r1: "D".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0x1D, r1: "E".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0x25, r1: "H".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0x2D, r1: "L".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0x35, r1: "(HL)".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0x3D, r1: "A".to_string(), r2: "".to_string()}),
                )]
                fn test(arg: Args) {
                    let mut cpu = common::fixture::setup_cpu();

                    let opcode = &OPCODES[arg.opcode as usize];
                    assert_eq!(opcode.r1, arg.r1);
                    assert_eq!(opcode.r2, arg.r2);

                    let handler = &opcode.handler;

                    // no carry
                    cpu.reg.F.unpack(0b11110000);
                    cpu.store(&opcode.r1, 0x11);
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    assert_eq!(cpu.load(&opcode.r1), 0x10);
                    assert_eq!(cpu.reg.F.pack(), 0b01010000);

                    // half carry
                    cpu.reg.F.unpack(0b11110000);
                    cpu.store(&opcode.r1, 0x10);
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    assert_eq!(cpu.load(&opcode.r1), 0x0F);
                    assert_eq!(cpu.reg.F.pack(), 0b01110000);

                    // zero
                    cpu.reg.F.unpack(0b11110000);
                    cpu.store(&opcode.r1, 0x01);
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    assert_eq!(cpu.load(&opcode.r1), 0x00);
                    assert_eq!(cpu.reg.F.pack(), 0b11010000);
                }
            }

            describe "decrr" {
                struct Args {
                    opcode: Byte,
                    r1: String,
                    r2: String,
                }
                #[rstest(arg,
                    case(Args{opcode: 0x0B, r1: "BC".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0x1B, r1: "DE".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0x2B, r1: "HL".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0x3B, r1: "SP".to_string(), r2: "".to_string()}),
                )]
                fn test(arg: Args) {
                    let mut cpu = common::fixture::setup_cpu();

                    let opcode = &OPCODES[arg.opcode as usize];
                    assert_eq!(opcode.r1, arg.r1);
                    assert_eq!(opcode.r2, arg.r2);

                    let handler = &opcode.handler;

                    cpu.store(&opcode.r1, 0x1234);
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    assert_eq!(cpu.load(&opcode.r1), 0x1233);
                }
            }
        }

        describe "AND" {
            describe "and" {
                struct Args {
                    opcode: Byte,
                    r1: String,
                    r2: String,
                }
                #[rstest(arg,
                    case(Args{opcode: 0xA0, r1: "B".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xA1, r1: "C".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xA2, r1: "D".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xA3, r1: "E".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xA4, r1: "H".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xA5, r1: "L".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xA6, r1: "(HL)".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xA7, r1: "A".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xE6, r1: "d".to_string(), r2: "".to_string()}),
                )]
                fn test(arg: Args) {
                    let mut cpu = common::fixture::setup_cpu();

                    let opcode = &OPCODES[arg.opcode as usize];
                    assert_eq!(opcode.r1, arg.r1);
                    assert_eq!(opcode.r2, arg.r2);

                    let handler = &opcode.handler;

                    // equal
                    cpu.reg.A = 0b11110000;
                    if opcode.r1.as_str() == "d" {
                        cpu.bus.write(cpu.reg.PC,  0b11110000);
                    } else {
                        cpu.store(&opcode.r1,  0b11110000);
                    }
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    assert_eq!(cpu.load(&"A".to_string()), 0b11110000);
                    assert_eq!(cpu.reg.F.pack(), 0b00100000);

                    // oposite
                    cpu.reg.A = 0b11110000;
                    if opcode.r1.as_str() == "d" {
                        cpu.bus.write(cpu.reg.PC,  0b00001111);
                    } else {
                        cpu.store(&opcode.r1,  0b00001111);
                    }
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());

                    if opcode.r1.as_str() != "A" {
                        assert_eq!(cpu.load(&"A".to_string()), 0b00000000);
                        assert_eq!(cpu.reg.F.pack(), 0b10100000);
                    }

                    // other
                    cpu.reg.A = 0b11110000;
                    if opcode.r1.as_str() == "d" {
                        cpu.bus.write(cpu.reg.PC,  0b10100000);
                    } else {
                        cpu.store(&opcode.r1,  0b10100000);
                    }
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    
                    if opcode.r1.as_str() != "A" {
                        assert_eq!(cpu.load(&"A".to_string()), 0b10100000);
                        assert_eq!(cpu.reg.F.pack(), 0b00100000);
                    }
                }
            }
        }

        describe "OR" {
            describe "or" {
                struct Args {
                    opcode: Byte,
                    r1: String,
                    r2: String,
                }
                #[rstest(arg,
                    case(Args{opcode: 0xB0, r1: "B".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xB1, r1: "C".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xB2, r1: "D".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xB3, r1: "E".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xB4, r1: "H".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xB5, r1: "L".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xB6, r1: "(HL)".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xB7, r1: "A".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xF6, r1: "d".to_string(), r2: "".to_string()}),
                )]
                fn test(arg: Args) {
                    let mut cpu = common::fixture::setup_cpu();

                    let opcode = &OPCODES[arg.opcode as usize];
                    assert_eq!(opcode.r1, arg.r1);
                    assert_eq!(opcode.r2, arg.r2);

                    let handler = &opcode.handler;

                    // equal
                    cpu.reg.A = 0b11110000;
                    if opcode.r1.as_str() == "d" {
                        cpu.bus.write(cpu.reg.PC,  0b11110000);
                    } else {
                        cpu.store(&opcode.r1,  0b11110000);
                    }
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    assert_eq!(cpu.load(&"A".to_string()), 0b11110000);
                    assert_eq!(cpu.reg.F.pack(), 0b00000000);

                    // oposite
                    cpu.reg.A = 0b11110000;
                    if opcode.r1.as_str() == "d" {
                        cpu.bus.write(cpu.reg.PC,  0b00001111);
                    } else {
                        cpu.store(&opcode.r1,  0b00001111);
                    }
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());

                    if opcode.r1.as_str() != "A" {
                        assert_eq!(cpu.load(&"A".to_string()), 0b11111111);
                        assert_eq!(cpu.reg.F.pack(), 0b00000000);
                    }

                    // other
                    cpu.reg.A = 0b11110000;
                    if opcode.r1.as_str() == "d" {
                        cpu.bus.write(cpu.reg.PC,  0b00000101);
                    } else {
                        cpu.store(&opcode.r1,  0b00000101);
                    }
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    
                    if opcode.r1.as_str() != "A" {
                        assert_eq!(cpu.load(&"A".to_string()), 0b11110101);
                        assert_eq!(cpu.reg.F.pack(), 0b00000000);
                    }

                    // zero
                    cpu.reg.A = 0b00000000;
                    if opcode.r1.as_str() == "d" {
                        cpu.bus.write(cpu.reg.PC,  0b00000000);
                    } else {
                        cpu.store(&opcode.r1,  0b00000000);
                    }
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    
                    if opcode.r1.as_str() != "A" {
                        assert_eq!(cpu.load(&"A".to_string()), 0b00000000);
                        assert_eq!(cpu.reg.F.pack(), 0b10000000);
                    }
                }
            }
        }

        describe "XOR" {
            describe "xor" {
                struct Args {
                    opcode: Byte,
                    r1: String,
                    r2: String,
                }
                #[rstest(arg,
                    case(Args{opcode: 0xA8, r1: "B".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xA9, r1: "C".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xAA, r1: "D".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xAB, r1: "E".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xAC, r1: "H".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xAD, r1: "L".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xAE, r1: "(HL)".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xAF, r1: "A".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xEE, r1: "d".to_string(), r2: "".to_string()}),
                )]
                fn test(arg: Args) {
                    let mut cpu = common::fixture::setup_cpu();

                    let opcode = &OPCODES[arg.opcode as usize];
                    assert_eq!(opcode.r1, arg.r1);
                    assert_eq!(opcode.r2, arg.r2);

                    let handler = &opcode.handler;

                    // equal
                    cpu.reg.A = 0b11110000;
                    if opcode.r1.as_str() == "d" {
                        cpu.bus.write(cpu.reg.PC,  0b11110000);
                    } else {
                        cpu.store(&opcode.r1,  0b11110000);
                    }
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    assert_eq!(cpu.load(&"A".to_string()), 0b00000000);
                    assert_eq!(cpu.reg.F.pack(), 0b10000000);

                    // oposite
                    cpu.reg.A = 0b11110000;
                    if opcode.r1.as_str() == "d" {
                        cpu.bus.write(cpu.reg.PC,  0b00001111);
                    } else {
                        cpu.store(&opcode.r1,  0b00001111);
                    }
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());

                    if opcode.r1.as_str() != "A" {
                        assert_eq!(cpu.load(&"A".to_string()), 0b11111111);
                        assert_eq!(cpu.reg.F.pack(), 0b00000000);
                    }

                    // other
                    cpu.reg.A = 0b11110000;
                    if opcode.r1.as_str() == "d" {
                        cpu.bus.write(cpu.reg.PC,  0b01010101);
                    } else {
                        cpu.store(&opcode.r1,  0b01010101);
                    }
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    
                    if opcode.r1.as_str() != "A" {
                        assert_eq!(cpu.load(&"A".to_string()), 0b10100101);
                        assert_eq!(cpu.reg.F.pack(), 0b00000000);
                    }

                    // zero
                    cpu.reg.A = 0b00000000;
                    if opcode.r1.as_str() == "d" {
                        cpu.bus.write(cpu.reg.PC,  0b00000000);
                    } else {
                        cpu.store(&opcode.r1,  0b00000000);
                    }
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    
                    if opcode.r1.as_str() != "A" {
                        assert_eq!(cpu.load(&"A".to_string()), 0b00000000);
                        assert_eq!(cpu.reg.F.pack(), 0b10000000);
                    }
                }
            }
        }

        describe "CP" {
            describe "cp" {
                struct Args {
                    opcode: Byte,
                    r1: String,
                    r2: String,
                }
                #[rstest(arg,
                    case(Args{opcode: 0xB8, r1: "B".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xB9, r1: "C".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xBA, r1: "D".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xBB, r1: "E".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xBC, r1: "H".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xBD, r1: "L".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xBE, r1: "(HL)".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xBF, r1: "A".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xFE, r1: "d".to_string(), r2: "".to_string()}),
                )]
                fn test(arg: Args) {
                    let mut cpu = common::fixture::setup_cpu();

                    let opcode = &OPCODES[arg.opcode as usize];
                    assert_eq!(opcode.r1, arg.r1);
                    assert_eq!(opcode.r2, arg.r2);

                    let handler = &opcode.handler;

                    // equal
                    cpu.reg.A = 0x12;
                    if opcode.r1.as_str() == "d" {
                        cpu.bus.write(cpu.reg.PC,  0x12);
                    } else {
                        cpu.store(&opcode.r1,  0x12);
                    }
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    assert_eq!(cpu.reg.F.pack(), 0b11000000);

                    // greater than A
                    cpu.reg.A = 0x12;
                    if opcode.r1.as_str() == "d" {
                        cpu.bus.write(cpu.reg.PC,  0x13);
                    } else {
                        cpu.store(&opcode.r1,  0x13);
                    }
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());

                    if opcode.r1.as_str() != "A" {
                        assert_eq!(cpu.reg.F.pack(), 0b01110000);
                    }

                    // less than A with borrow
                    cpu.reg.A = 0x12;
                    if opcode.r1.as_str() == "d" {
                        cpu.bus.write(cpu.reg.PC,  0x03);
                    } else {
                        cpu.store(&opcode.r1,  0x03);
                    }
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());

                    if opcode.r1.as_str() != "A" {
                        assert_eq!(cpu.reg.F.pack(), 0b01100000);
                    }

                    // less than A with no borrow
                    cpu.reg.A = 0x12;
                    if opcode.r1.as_str() == "d" {
                        cpu.bus.write(cpu.reg.PC,  0x02);
                    } else {
                        cpu.store(&opcode.r1,  0x02);
                    }
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());

                    if opcode.r1.as_str() != "A" {
                        assert_eq!(cpu.reg.F.pack(), 0b01000000);
                    }
                }
            }
        }

        describe "CALL" {
            describe "call" {
                struct Args {
                    opcode: Byte,
                    r1: String,
                    r2: String,
                }
                #[rstest(arg,
                    case(Args{opcode: 0xCD, r1: "aa".to_string(), r2: "".to_string()}),
                )]
                fn test(arg: Args) {
                    let mut cpu = common::fixture::setup_cpu();

                    let opcode = &OPCODES[arg.opcode as usize];
                    assert_eq!(opcode.r1, arg.r1);
                    assert_eq!(opcode.r2, arg.r2);

                    let handler = &opcode.handler;

                    cpu.reg.PC = 0x5678;
                    cpu.reg.SP = 0xFFFC;
                    cpu.bus.write(cpu.reg.PC,  0x34);
                    cpu.bus.write(cpu.reg.PC + 1,  0x12);
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    assert_eq!(cpu.load(&"PC".to_string()), 0x1234);
                    assert_eq!(cpu.load(&"SP".to_string()), 0xFFFA);
                    assert_eq!(cpu.bus.read(0xFFFA), 0x7A);
                    assert_eq!(cpu.bus.read(0xFFFB), 0x56);
                }
            }

            describe "callf" {
                struct Args {
                    opcode: Byte,
                    r1: String,
                    r2: String,
                }
                #[rstest(arg,
                    case(Args{opcode: 0xC4, r1: "NZ".to_string(), r2: "aa".to_string()}),
                    case(Args{opcode: 0xCC, r1: "Z".to_string(), r2: "aa".to_string()}),
                    case(Args{opcode: 0xD4, r1: "NC".to_string(), r2: "aa".to_string()}),
                    case(Args{opcode: 0xDC, r1: "C".to_string(), r2: "aa".to_string()}),
                )]
                fn test(arg: Args) {
                    let mut cpu = common::fixture::setup_cpu();

                    let opcode = &OPCODES[arg.opcode as usize];
                    assert_eq!(opcode.r1, arg.r1);
                    assert_eq!(opcode.r2, arg.r2);

                    let handler = &opcode.handler;

                    // all true
                    cpu.reg.PC = 0x5678;
                    cpu.reg.SP = 0xFFFC;
                    cpu.bus.write(cpu.reg.PC,  0x34);
                    cpu.bus.write(cpu.reg.PC + 1,  0x12);
                    cpu.reg.F.unpack(0xF0);
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    if opcode.r1.as_str() == "NZ" || opcode.r1.as_str() == "NC" {
                        assert_eq!(cpu.load(&"PC".to_string()), 0x567A);
                        assert_eq!(cpu.load(&"SP".to_string()), 0xFFFC);
                    } else {
                        assert_eq!(cpu.load(&"PC".to_string()), 0x1234);
                        assert_eq!(cpu.load(&"SP".to_string()), 0xFFFA);
                        assert_eq!(cpu.bus.read(0xFFFA), 0x7A);
                        assert_eq!(cpu.bus.read(0xFFFB), 0x56);
                    }

                    // all false
                    cpu.reg.PC = 0x5678;
                    cpu.reg.SP = 0xFFFC;
                    cpu.bus.write(cpu.reg.PC,  0x34);
                    cpu.bus.write(cpu.reg.PC + 1,  0x12);
                    cpu.reg.F.unpack(0x00);
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    if opcode.r1.as_str() == "NZ" || opcode.r1.as_str() == "NC" {
                        assert_eq!(cpu.load(&"PC".to_string()), 0x1234);
                        assert_eq!(cpu.load(&"SP".to_string()), 0xFFFA);
                        assert_eq!(cpu.bus.read(0xFFFA), 0x7A);
                        assert_eq!(cpu.bus.read(0xFFFB), 0x56);
                    } else {
                        assert_eq!(cpu.load(&"PC".to_string()), 0x567A);
                        assert_eq!(cpu.load(&"SP".to_string()), 0xFFFC);
                    }
                }
            }
        }

        describe "RET" {
            describe "ret" {
                struct Args {
                    opcode: Byte,
                    r1: String,
                    r2: String,
                }
                #[rstest(arg,
                    case(Args{opcode: 0xC0, r1: "NZ".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xC8, r1: "Z".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xC9, r1: "".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xD0, r1: "NC".to_string(), r2: "".to_string()}),
                    case(Args{opcode: 0xD8, r1: "C".to_string(), r2: "".to_string()}),
                )]
                fn test(arg: Args) {
                    let mut cpu = common::fixture::setup_cpu();

                    let opcode = &OPCODES[arg.opcode as usize];
                    assert_eq!(opcode.r1, arg.r1);
                    assert_eq!(opcode.r2, arg.r2);

                    cpu.reg.SP = 0xFFFC;
                    cpu.reg.PC = 0x5678;
                    cpu.bus.write(cpu.reg.SP, 0x34);
                    cpu.bus.write(cpu.reg.SP + 1, 0x12);

                    let handler = &opcode.handler;

                    // all positive
                    cpu.reg.F.unpack(0xF0);
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    if opcode.r1.as_str() == "NZ" || opcode.r1.as_str() == "NC" {
                        assert_eq!(cpu.load(&"SP".to_string()), 0xFFFC);
                        assert_eq!(cpu.load(&"PC".to_string()), 0x5678);
                    } else {
                        assert_eq!(cpu.load(&"SP".to_string()), 0xFFFE);
                        assert_eq!(cpu.load(&"PC".to_string()), 0x1234);
                    }

                    // all negative
                    cpu.reg.SP = 0xFFFC;
                    cpu.reg.PC = 0x5678;
                    cpu.reg.F.unpack(0x00);
                    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                    if opcode.r1.as_str() == "NZ" || opcode.r1.as_str() == "NC" || opcode.r1.as_str() == "" {
                        assert_eq!(cpu.load(&"SP".to_string()), 0xFFFE);
                        assert_eq!(cpu.load(&"PC".to_string()), 0x1234);
                    } else {
                        assert_eq!(cpu.load(&"SP".to_string()), 0xFFFC);
                        assert_eq!(cpu.load(&"PC".to_string()), 0x5678);
                    }
                }
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
        describe "set" {
            struct Args {
                opcode: Byte,
                i: String,
                r1: String,
            }
            #[rstest(arg,
                case(Args{opcode: 0xC0, i: "0".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0xC1, i: "0".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0xC2, i: "0".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0xC3, i: "0".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0xC4, i: "0".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0xC5, i: "0".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0xC6, i: "0".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0xC7, i: "0".to_string(), r1: "A".to_string()}),
                case(Args{opcode: 0xC8, i: "1".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0xC9, i: "1".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0xCA, i: "1".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0xCB, i: "1".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0xCC, i: "1".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0xCD, i: "1".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0xCE, i: "1".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0xCF, i: "1".to_string(), r1: "A".to_string()}),
                case(Args{opcode: 0xD0, i: "2".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0xD1, i: "2".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0xD2, i: "2".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0xD3, i: "2".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0xD4, i: "2".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0xD5, i: "2".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0xD6, i: "2".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0xD7, i: "2".to_string(), r1: "A".to_string()}),
                case(Args{opcode: 0xD8, i: "3".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0xD9, i: "3".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0xDA, i: "3".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0xDB, i: "3".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0xDC, i: "3".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0xDD, i: "3".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0xDE, i: "3".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0xDF, i: "3".to_string(), r1: "A".to_string()}),
                case(Args{opcode: 0xE0, i: "4".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0xE1, i: "4".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0xE2, i: "4".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0xE3, i: "4".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0xE4, i: "4".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0xE5, i: "4".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0xE6, i: "4".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0xE7, i: "4".to_string(), r1: "A".to_string()}),
                case(Args{opcode: 0xE8, i: "5".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0xE9, i: "5".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0xEA, i: "5".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0xEB, i: "5".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0xEC, i: "5".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0xED, i: "5".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0xEE, i: "5".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0xEF, i: "5".to_string(), r1: "A".to_string()}),
                case(Args{opcode: 0xF0, i: "6".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0xF1, i: "6".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0xF2, i: "6".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0xF3, i: "6".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0xF4, i: "6".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0xF5, i: "6".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0xF6, i: "6".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0xF7, i: "6".to_string(), r1: "A".to_string()}),
                case(Args{opcode: 0xF8, i: "7".to_string(), r1: "B".to_string()}),
                case(Args{opcode: 0xF9, i: "7".to_string(), r1: "C".to_string()}),
                case(Args{opcode: 0xFA, i: "7".to_string(), r1: "D".to_string()}),
                case(Args{opcode: 0xFB, i: "7".to_string(), r1: "E".to_string()}),
                case(Args{opcode: 0xFC, i: "7".to_string(), r1: "H".to_string()}),
                case(Args{opcode: 0xFD, i: "7".to_string(), r1: "L".to_string()}),
                case(Args{opcode: 0xFE, i: "7".to_string(), r1: "(HL)".to_string()}),
                case(Args{opcode: 0xFF, i: "7".to_string(), r1: "A".to_string()}),
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
                assert_eq!(rust_boy::util::bit(&(cpu.load(&opcode.r2) as Byte), &i), 1);

                before = 0b00000000u8;
                cpu.store(&opcode.r2, before as Word);
                let handler = &opcode.handler;
                handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());
                assert_eq!(rust_boy::util::bit(&(cpu.load(&opcode.r2) as Byte), &i), 1);
            }
        }
    }
}

fn cb_helper(mut cpu: Cpu, opcode: &OpCode, before: Byte, want: Byte, flag: Byte) -> Cpu {
    cpu.store(&opcode.r1, before as Word);
    let handler = &opcode.handler;
    handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());

    assert_eq!(cpu.reg.F.pack(), flag);
    let rr_array = ["BC", "DE", "HL", "AF", "HL", "SP"];
    if rr_array.contains(&opcode.r1.as_str()) {
        assert_eq!(cpu.load(&opcode.r1), bytes_2_word(want + 1, want));
    } else {
        assert_eq!(cpu.load(&opcode.r1), want as Word);
    }

    cpu
}
