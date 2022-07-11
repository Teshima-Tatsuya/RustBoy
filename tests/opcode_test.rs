extern crate rstest;
#[cfg(test)]
extern crate speculate;

mod common;
use rstest::*;
use rust_boy::cpu::Cpu;
use rust_boy::opcode::OpCode;
use speculate::speculate;

speculate! {
    describe "LD" {
        describe "LD R1, R1" {
            #[test]
            fn test() {
                let mut cpu = common::fixture::setup_cpu();
                cpu.step();

            }

        }
    }
}
