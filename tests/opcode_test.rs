extern crate rstest;
#[cfg(test)]
extern crate speculate;

mod common;
use rstest::*;
use rust_boy::cpu::*;
use rust_boy::opcode::*;
use rust_boy::types::*;
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
                case(Args{opcode: 0x40, r1: "B".to_string(), r2: "B".to_string()}),
                case(Args{opcode: 0x41, r1: "B".to_string(), r2: "C".to_string()}),
                case(Args{opcode: 0x42, r1: "B".to_string(), r2: "D".to_string()}),
                case(Args{opcode: 0x43, r1: "B".to_string(), r2: "E".to_string()}),
                case(Args{opcode: 0x44, r1: "B".to_string(), r2: "H".to_string()}),
                case(Args{opcode: 0x45, r1: "B".to_string(), r2: "L".to_string()}),
                case(Args{opcode: 0x47, r1: "B".to_string(), r2: "A".to_string()}),
                case(Args{opcode: 0x48, r1: "C".to_string(), r2: "B".to_string()}),
                case(Args{opcode: 0x49, r1: "C".to_string(), r2: "C".to_string()}),
                case(Args{opcode: 0x4A, r1: "C".to_string(), r2: "D".to_string()}),
                case(Args{opcode: 0x4B, r1: "C".to_string(), r2: "E".to_string()}),
                case(Args{opcode: 0x4C, r1: "C".to_string(), r2: "H".to_string()}),
                case(Args{opcode: 0x4D, r1: "C".to_string(), r2: "L".to_string()}),
                case(Args{opcode: 0x4F, r1: "C".to_string(), r2: "A".to_string()}),
                case(Args{opcode: 0x50, r1: "D".to_string(), r2: "B".to_string()}),
                case(Args{opcode: 0x51, r1: "D".to_string(), r2: "C".to_string()}),
                case(Args{opcode: 0x52, r1: "D".to_string(), r2: "D".to_string()}),
                case(Args{opcode: 0x53, r1: "D".to_string(), r2: "E".to_string()}),
                case(Args{opcode: 0x54, r1: "D".to_string(), r2: "H".to_string()}),
                case(Args{opcode: 0x55, r1: "D".to_string(), r2: "L".to_string()}),
                case(Args{opcode: 0x57, r1: "D".to_string(), r2: "A".to_string()}),
                case(Args{opcode: 0x58, r1: "E".to_string(), r2: "B".to_string()}),
                case(Args{opcode: 0x59, r1: "E".to_string(), r2: "C".to_string()}),
                case(Args{opcode: 0x5A, r1: "E".to_string(), r2: "D".to_string()}),
                case(Args{opcode: 0x5B, r1: "E".to_string(), r2: "E".to_string()}),
                case(Args{opcode: 0x5C, r1: "E".to_string(), r2: "H".to_string()}),
                case(Args{opcode: 0x5D, r1: "E".to_string(), r2: "L".to_string()}),
                case(Args{opcode: 0x5F, r1: "E".to_string(), r2: "A".to_string()}),
                case(Args{opcode: 0x60, r1: "H".to_string(), r2: "B".to_string()}),
                case(Args{opcode: 0x61, r1: "H".to_string(), r2: "C".to_string()}),
                case(Args{opcode: 0x62, r1: "H".to_string(), r2: "D".to_string()}),
                case(Args{opcode: 0x63, r1: "H".to_string(), r2: "E".to_string()}),
                case(Args{opcode: 0x64, r1: "H".to_string(), r2: "H".to_string()}),
                case(Args{opcode: 0x65, r1: "H".to_string(), r2: "L".to_string()}),
                case(Args{opcode: 0x67, r1: "H".to_string(), r2: "A".to_string()}),
                case(Args{opcode: 0x68, r1: "L".to_string(), r2: "B".to_string()}),
                case(Args{opcode: 0x69, r1: "L".to_string(), r2: "C".to_string()}),
                case(Args{opcode: 0x6A, r1: "L".to_string(), r2: "D".to_string()}),
                case(Args{opcode: 0x6B, r1: "L".to_string(), r2: "E".to_string()}),
                case(Args{opcode: 0x6C, r1: "L".to_string(), r2: "H".to_string()}),
                case(Args{opcode: 0x6D, r1: "L".to_string(), r2: "L".to_string()}),
                case(Args{opcode: 0x6F, r1: "L".to_string(), r2: "A".to_string()}),
                case(Args{opcode: 0xF2, r1: "A".to_string(), r2: "(C)".to_string()}),
            )]
            fn test(arg: Args) {
                let mut cpu = common::fixture::setup_cpu();

                let opcode = &OPCODES[arg.opcode as usize];
                let want = 0x12;

                if &arg.r2 >= &"A".to_string() && &arg.r2 <= &"L".to_string() {
                    // r
                    cpu.reg.r_mut(&arg.r2, want);
                } else if &arg.r2 == &"(C)".to_string() {
                    // (m)
                    cpu.reg.C = want;
                    let addr = 0xFF00 | cpu.reg.C as Word;
                    cpu.bus.write(addr, want);
                }

                let handler = &opcode.handler;
                handler(&mut cpu, opcode.r1.to_string(), opcode.r2.to_string());

                assert_eq!(opcode.r1, arg.r1);
                assert_eq!(opcode.r2, arg.r2);
                assert_eq!(cpu.load(&opcode.r1), want as Word);
                assert_eq!(cpu.load(&opcode.r2), want as Word);
            }
        }
    }
    describe "JP" {
        describe "jpa16" {

        }
    }
}
